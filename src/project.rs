use serde::{Deserialize, Serialize};
use rusqlite::{Connection, params};
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SavedLook { pub name: String, pub intensity: f32, pub rgb: [f32;3], pub seconds: f32 }
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Project { pub name: String, pub looks: Vec<SavedLook> }
impl Project {
 pub fn validate(&self)->Result<(),String>{
  if self.name.trim().is_empty() || self.name.len()>256 || self.looks.len()>1000 {return Err("Некорректный проект".into());}
  for l in &self.looks { crate::lighting::Look{intensity:l.intensity,rgb:l.rgb}.validate()?;
   if l.name.trim().is_empty() || !l.seconds.is_finite() || !(0.0..=600.0).contains(&l.seconds){return Err("Некорректная картина или переход".into());}
  } Ok(())
 }
}
pub struct Store(Connection);
impl Store {
 pub fn open(path: &std::path::Path)->Result<Self,String>{
 let c=Connection::open(path).map_err(|e|e.to_string())?;
 c.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON; BEGIN; CREATE TABLE IF NOT EXISTS projects(id INTEGER PRIMARY KEY CHECK(id=1), body TEXT NOT NULL); CREATE TABLE IF NOT EXISTS history(id INTEGER PRIMARY KEY, body TEXT NOT NULL, created_at TEXT DEFAULT CURRENT_TIMESTAMP); PRAGMA user_version=1; COMMIT;").map_err(|e|e.to_string())?;Ok(Self(c))
 }
 pub fn save(&mut self,p:&Project)->Result<(),String>{p.validate()?;let body=serde_json::to_string(p).map_err(|e|e.to_string())?;let tx=self.0.transaction().map_err(|e|e.to_string())?;
 tx.execute("INSERT INTO history(body) SELECT body FROM projects WHERE id=1",[]).map_err(|e|e.to_string())?;
 tx.execute("INSERT INTO projects(id,body) VALUES(1,?1) ON CONFLICT(id) DO UPDATE SET body=excluded.body",params![body]).map_err(|e|e.to_string())?;tx.commit().map_err(|e|e.to_string())
 }
 pub fn load(&self)->Result<Option<Project>,String>{use rusqlite::OptionalExtension;let body:Option<String>=self.0.query_row("SELECT body FROM projects WHERE id=1",[],|r|r.get(0)).optional().map_err(|e|e.to_string())?;body.map(|b|{let p:Project=serde_json::from_str(&b).map_err(|e|e.to_string())?;p.validate()?;Ok(p)}).transpose()}
}
impl Store {
 pub fn save_venue(&self,v:&crate::venue::Venue)->Result<(),String>{v.validate()?;self.0.execute_batch("CREATE TABLE IF NOT EXISTS venue(id INTEGER PRIMARY KEY CHECK(id=1),body TEXT NOT NULL)").map_err(|e|e.to_string())?;let body=serde_json::to_string(v).map_err(|e|e.to_string())?;self.0.execute("INSERT INTO venue VALUES(1,?1) ON CONFLICT(id) DO UPDATE SET body=excluded.body",params![body]).map_err(|e|e.to_string())?;Ok(())}
 pub fn load_venue(&self)->Result<Option<crate::venue::Venue>,String>{use rusqlite::OptionalExtension;self.0.execute_batch("CREATE TABLE IF NOT EXISTS venue(id INTEGER PRIMARY KEY CHECK(id=1),body TEXT NOT NULL)").map_err(|e|e.to_string())?;let body:Option<String>=self.0.query_row("SELECT body FROM venue WHERE id=1",[],|r|r.get(0)).optional().map_err(|e|e.to_string())?;body.map(|b|{let v:crate::venue::Venue=serde_json::from_str(&b).map_err(|e|e.to_string())?;v.validate()?;Ok(v)}).transpose()}
}
impl Store{
 pub fn save_cues(&self,cues:&[crate::playback::Cue])->Result<(),String>{if cues.len()>1000{return Err("Слишком много Cue".into());}for c in cues{c.validate()?;}self.0.execute_batch("CREATE TABLE IF NOT EXISTS cues(id INTEGER PRIMARY KEY CHECK(id=1),body TEXT NOT NULL)").map_err(|e|e.to_string())?;let body=serde_json::to_string(cues).map_err(|e|e.to_string())?;self.0.execute("INSERT INTO cues VALUES(1,?1) ON CONFLICT(id) DO UPDATE SET body=excluded.body",params![body]).map_err(|e|e.to_string())?;Ok(())}
 pub fn load_cues(&self)->Result<Vec<crate::playback::Cue>,String>{use rusqlite::OptionalExtension;self.0.execute_batch("CREATE TABLE IF NOT EXISTS cues(id INTEGER PRIMARY KEY CHECK(id=1),body TEXT NOT NULL)").map_err(|e|e.to_string())?;let body:Option<String>=self.0.query_row("SELECT body FROM cues WHERE id=1",[],|r|r.get(0)).optional().map_err(|e|e.to_string())?;let cues:Vec<crate::playback::Cue>=serde_json::from_str(body.as_deref().unwrap_or("[]")).map_err(|e|e.to_string())?;for c in &cues{c.validate()?;}Ok(cues)}
}
impl Store {
 fn recognition_schema(&self)->Result<(),String>{self.0.execute_batch("CREATE TABLE IF NOT EXISTS recognition_profiles(id TEXT NOT NULL, revision INTEGER NOT NULL, body TEXT NOT NULL, PRIMARY KEY(id,revision)); CREATE TABLE IF NOT EXISTS recognition_evidence(id INTEGER PRIMARY KEY, fixture TEXT NOT NULL, profile_id TEXT NOT NULL, revision INTEGER NOT NULL, note TEXT NOT NULL, created_at TEXT DEFAULT CURRENT_TIMESTAMP);").map_err(|e|e.to_string())}
 pub fn import_fingerprint(&self,p:&crate::recognition::Fingerprint)->Result<(),String>{p.validate()?;self.recognition_schema()?;let body=serde_json::to_string(p).map_err(|e|e.to_string())?;self.0.execute("INSERT INTO recognition_profiles(id,revision,body) VALUES(?1,?2,?3)",params![p.id,p.revision,body]).map_err(|e|format!("Не удалось сохранить профиль; для обновления используйте новую версию: {e}"))?;Ok(())}
 pub fn fingerprints(&self)->Result<Vec<crate::recognition::Fingerprint>,String>{self.recognition_schema()?;let mut q=self.0.prepare("SELECT body FROM recognition_profiles ORDER BY id,revision").map_err(|e|e.to_string())?;let rows=q.query_map([],|r|r.get::<_,String>(0)).map_err(|e|e.to_string())?;rows.map(|r|crate::recognition::Fingerprint::parse(&r.map_err(|e|e.to_string())?)).collect()}
 pub fn search_fingerprint(&self,query:&crate::recognition::Fingerprint)->Result<Vec<crate::recognition::Comparison>,String>{query.validate()?;self.fingerprints()?.iter().map(|p|crate::recognition::compare(query,p)).collect()}
 pub fn note_candidate(&self,fixture:&str,id:&str,revision:u32,note:&str)->Result<(),String>{if note.trim().is_empty()||note.len()>2000{return Err("Укажите основание сопоставления".into());}let v=self.load_venue()?.ok_or("Сначала сохраните площадку")?;if !v.fixtures.iter().any(|f|f.id==fixture){return Err("Прибор отсутствует на площадке".into());}if !self.fingerprints()?.iter().any(|p|p.id==id&&p.revision==revision){return Err("Профиль не найден".into());}self.0.execute("INSERT INTO recognition_evidence(fixture,profile_id,revision,note) VALUES(?1,?2,?3,?4)",params![fixture,id,revision,note]).map_err(|e|e.to_string())?;Ok(())}
}
