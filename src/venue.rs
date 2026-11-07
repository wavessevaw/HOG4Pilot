//! Venue mapping is metadata-only; identification output is synthetic and gated.
use serde::{Deserialize,Serialize};
use crate::{fixture::virtual_rgb,lighting::Look,safety::SafetyEngine};
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub enum Answer<T>{ Pending, Unknown, Known(T) }
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct MappedFixture{pub id:String,pub address:usize,#[serde(default)] pub universe:u16,#[serde(default="default_channels")] pub channels:usize,pub verified:bool,pub hazer:bool,#[serde(default="pending")] pub identification:Answer<String>,pub mount:Answer<String>,pub position:Answer<String>,pub role:Answer<String>,pub coverage:Answer<String>}
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Venue{pub name:String,pub simulation:bool,pub fixtures:Vec<MappedFixture>,pub cursor:usize}
#[derive(Serialize)]
pub struct Group{pub name:String,pub fixtures:Vec<String>}
impl Venue{
 pub fn demo()->Self{Self{name:"Демонстрационная площадка • 24 прибора".into(),simulation:true,cursor:0,fixtures:(0..24).map(|i|MappedFixture{id:format!("Прибор {:02}",i+1),address:i*4+1,universe:0,channels:4,verified:i<22,hazer:i==21,identification:Answer::Pending,mount:Answer::Pending,position:Answer::Pending,role:Answer::Pending,coverage:Answer::Pending}).collect()}}
 pub fn validate(&self)->Result<(),String>{if !self.simulation {return self.validate_passive();}if self.fixtures.len()!=24 || self.cursor>24{return Err("Поддерживается только встроенная демонстрационная площадка".into());}let demo=Self::demo();for(f,d)in self.fixtures.iter().zip(demo.fixtures){if f.id!=d.id || f.address!=d.address || f.universe!=d.universe || f.channels!=d.channels || f.verified!=d.verified || f.hazer!=d.hazer{return Err("Изменены защищённые данные профиля".into());}}Ok(())}
 pub fn answer(&mut self,field:&str,value:Option<String>)->Result<(),String>{let f=self.fixtures.get_mut(self.cursor).ok_or("Настройка завершена")?;let answer=match value{Some(v)if !v.trim().is_empty() && v.len()<=100=>Answer::Known(v),Some(_)=>return Err("Некорректное значение".into()),None=>Answer::Unknown};match field{"identification"=>f.identification=answer,"mount"=>f.mount=answer,"position"=>f.position=answer,"role"=>f.role=answer,"coverage"=>{if matches!(answer,Answer::Known(_)) && (!f.verified||f.hazer){return Err("Для пассивного прибора доступно только «Я не знаю»".into());}f.coverage=answer},_=>return Err("Неизвестный этап".into())}Ok(())}
 pub fn next(&mut self)->Result<(),String>{let f=self.fixtures.get(self.cursor).ok_or("Настройка завершена")?;if matches!(f.mount,Answer::Pending)||matches!(f.position,Answer::Pending)||matches!(f.role,Answer::Pending)||matches!(f.coverage,Answer::Pending){return Err("Ответьте на этапы или выберите «Я не знаю»".into());}self.cursor+=1;Ok(())}
 pub fn identify(&self)->Result<Vec<u8>,String>{self.validate()?;let selected=self.fixtures.get(self.cursor).ok_or("Настройка завершена")?;if !selected.verified||selected.hazer{return Err("Пассивный режим: тестовое включение запрещено".into());}self.frame(&[selected.id.clone()],0.15,[1.0;3])}
 pub fn frame(&self,ids:&[String],intensity:f32,rgb:[f32;3])->Result<Vec<u8>,String>{self.validate()?;if !self.simulation{return Err("Импортированный патч находится в пассивном режиме".into());}Look{intensity,rgb}.validate()?;let mut result=vec![0;512];for id in ids{let f=self.fixtures.iter().find(|f| &f.id==id).ok_or("Прибор отсутствует")?;if !f.verified||f.hazer{return Err("Прибор не допущен к управлению".into());}let mut e=SafetyEngine::new(vec![virtual_rgb(&f.id,f.address)],Some([0;512]),0,"venue".into())?;let frame=e.render("venue",Look{intensity,rgb})?;result[f.address-1..f.address+3].copy_from_slice(&frame.values()[f.address-1..f.address+3]);}Ok(result)}
 pub fn groups(&self)->Vec<Group>{let mut groups=std::collections::BTreeMap::<String,Vec<String>>::new();for f in &self.fixtures{if !f.verified||f.hazer{continue;}if let Answer::Known(zone)=&f.coverage{groups.entry(zone.clone()).or_default().push(f.id.clone());}}groups.into_iter().map(|(name,fixtures)|Group{name,fixtures}).collect()}
}

fn pending()->Answer<String>{Answer::Pending}

fn default_channels()->usize{4}
impl Venue {
 pub fn import_patch(csv:&str)->Result<Self,String>{
  if csv.len()>1_000_000{return Err("Патч слишком большой".into());}
  let mut lines=csv.lines();if lines.next()!=Some("id,universe,address,channels,kind"){return Err("Ожидается заголовок id,universe,address,channels,kind".into());}
  let mut fixtures=Vec::new();
  for (i,line) in lines.enumerate(){if line.trim().is_empty(){continue;}let c:Vec<_>=line.split(',').map(str::trim).collect();if c.len()!=5{return Err(format!("Строка {}: ожидается 5 полей без запятых внутри",i+2));}
   if !["light","hazer","unknown"].contains(&c[4]){return Err("Тип: light, hazer или unknown".into());}
   fixtures.push(MappedFixture{id:c[0].into(),universe:c[1].parse().map_err(|_|"Неверная вселенная")?,address:c[2].parse().map_err(|_|"Неверный адрес")?,channels:c[3].parse().map_err(|_|"Неверное число каналов")?,verified:false,hazer:c[4]=="hazer",identification:Answer::Pending,mount:Answer::Pending,position:Answer::Pending,role:Answer::Pending,coverage:Answer::Pending});
  }
  let v=Self{name:"Импортированный патч • пассивный режим".into(),simulation:false,cursor:0,fixtures};v.validate()?;Ok(v)
 }
 fn validate_passive(&self)->Result<(),String>{
  if self.fixtures.is_empty()||self.fixtures.len()>4096||self.cursor>self.fixtures.len(){return Err("Неверный размер патча или позиция мастера".into());}
  let mut ids=std::collections::HashSet::new();let mut occupied=std::collections::HashSet::new();
  for f in &self.fixtures{
   if f.verified{return Err("Импорт не подтверждает профиль".into());}
   if f.id.trim().is_empty()||f.id.len()>100||!ids.insert(&f.id){return Err("Пустое или повторяющееся имя прибора".into());}
   if f.universe>32767||f.address==0||f.channels==0||f.channels>512||f.address.checked_add(f.channels).is_none_or(|end|end>513){return Err("Адрес или размер прибора вне диапазона".into());}
   for slot in f.address..f.address+f.channels{if !occupied.insert((f.universe,slot)){return Err(format!("Конфликт адресов: {}",f.id));}}
  }Ok(())
 }
}
