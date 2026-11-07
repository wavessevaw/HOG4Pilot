//! Offline structural recognition. A match is evidence, never output authorization.
use serde::{Deserialize, Serialize};
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="snake_case")]
pub enum Function { Intensity, Red, Green, Blue, White, Pan, Tilt, Shutter, Strobe, Zoom, Focus, Gobo, Prism, Colour, Reset, LampControl, Hazer, Unknown }
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct Range { pub min:u8, pub max:u8, pub function:Function, pub semantics:String }
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct Channel { pub offset:u16, pub fine_of:Option<u16>, pub neutral:Option<u8>, pub ranges:Vec<Range> }
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fingerprint { pub id:String, pub revision:u32, pub manufacturer:String, pub model:String, pub mode:String, pub source:String, pub channels:Vec<Channel> }
#[derive(Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="snake_case")]
pub enum Verdict { StructuralMatch, Incompatible, InsufficientData }
#[derive(Debug,Serialize,Deserialize)]
pub struct Comparison { pub candidate:String, pub revision:u32, pub verdict:Verdict, pub differences:Vec<String>, pub missing:Vec<String>, pub hazardous:bool }
impl Fingerprint {
 pub fn parse(json:&str)->Result<Self,String>{if json.len()>1_000_000{return Err("Профиль слишком большой".into());}let p:Self=serde_json::from_str(json).map_err(|e|format!("Некорректный JSON профиля: {e}"))?;p.validate()?;Ok(p)}
 pub fn validate(&self)->Result<(),String>{
  if self.id.trim().is_empty()||self.id.len()>128||self.revision==0||self.source.trim().is_empty()||self.source.len()>2000||self.model.len()>256||self.mode.len()>256||self.manufacturer.len()>256||self.channels.is_empty()||self.channels.len()>512{return Err("Нужны ID, версия, источник и 1–512 каналов".into());}
  let mut fine_parents=std::collections::HashSet::new();
  for(i,c)in self.channels.iter().enumerate(){if c.offset as usize!=i+1{return Err("Каналы должны идти подряд от 1".into());}
   if let Some(parent)=c.fine_of{if parent==0||parent as usize>self.channels.len()||parent==c.offset||!fine_parents.insert(parent){return Err("Некорректная или повторная привязка Fine".into());}if self.channels[parent as usize-1].fine_of.is_some(){return Err("Fine должен ссылаться на Coarse".into());}}
   let mut used=[false;256];for r in &c.ranges{if r.min>r.max||r.semantics.len()>256{return Err("Некорректный диапазон".into());}for slot in r.min..=r.max{if used[slot as usize]{return Err(format!("Канал {}: пересекающиеся диапазоны",c.offset));}used[slot as usize]=true;}}
  }Ok(())
 }
 pub fn missing(&self)->Vec<String>{let mut result=Vec::new();if self.mode.trim().is_empty(){result.push("Укажите режим прибора".into());}for c in &self.channels{if c.neutral.is_none(){result.push(format!("Канал {}: требуется подтверждённое нейтральное значение",c.offset));}let mut covered=[false;256];for r in &c.ranges{for n in r.min..=r.max{covered[n as usize]=true;}if r.function==Function::Unknown||r.semantics.trim().is_empty(){result.push(format!("Канал {}: уточните функцию и смысл диапазона {}–{}",c.offset,r.min,r.max));}}if covered.contains(&false){result.push(format!("Канал {}: таблица диапазонов неполная",c.offset));}}result}
 pub fn hazardous(&self)->bool{self.channels.iter().flat_map(|c|&c.ranges).any(|r|matches!(r.function,Function::Reset|Function::LampControl|Function::Hazer|Function::Unknown))}
}
pub fn compare(a:&Fingerprint,b:&Fingerprint)->Result<Comparison,String>{
 a.validate()?;b.validate()?;let mut differences=Vec::new();let mut missing=a.missing();missing.extend(b.missing().into_iter().map(|m|format!("Кандидат: {m}")));
 if a.channels.len()!=b.channels.len(){differences.push(format!("Число каналов: {} / {}",a.channels.len(),b.channels.len()));}
 for(x,y)in a.channels.iter().zip(&b.channels){if x.fine_of!=y.fine_of{differences.push(format!("Канал {}: различаются Coarse/Fine",x.offset));}if x.neutral.is_some()&&y.neutral.is_some()&&x.neutral!=y.neutral{differences.push(format!("Канал {}: разные нейтральные значения",x.offset));}
 let mut xr=x.ranges.clone();let mut yr=y.ranges.clone();xr.sort_by_key(|r|r.min);yr.sort_by_key(|r|r.min);
 if xr!=yr{differences.push(format!("Канал {}: различаются функции, диапазоны или их смысл",x.offset));}}
 let verdict=if !differences.is_empty(){Verdict::Incompatible}else if !missing.is_empty(){Verdict::InsufficientData}else{Verdict::StructuralMatch};
 Ok(Comparison{candidate:b.id.clone(),revision:b.revision,verdict,differences,missing,hazardous:a.hazardous()||b.hazardous()})
}
