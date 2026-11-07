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
