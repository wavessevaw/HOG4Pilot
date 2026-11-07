//! Deterministic time-based playback. Caller supplies monotonic elapsed time.
use crate::{lighting::Look,venue::Venue};
use serde::{Serialize,Deserialize};
#[derive(Clone,Serialize,Deserialize)]
pub struct FixtureState{pub id:String,pub intensity:f32,pub rgb:[f32;3]}
#[derive(Clone,Serialize,Deserialize)]
pub struct Cue{pub name:String,pub seconds:f32,pub fixtures:Vec<FixtureState>}
impl Cue{pub fn validate(&self)->Result<(),String>{if self.name.trim().is_empty()||!self.seconds.is_finite()||!(0.0..=600.0).contains(&self.seconds){return Err("Некорректное название или время Cue".into());}let mut ids=std::collections::HashSet::new();for f in &self.fixtures{if !ids.insert(&f.id){return Err("Повтор прибора в Cue".into());}Look{intensity:f.intensity,rgb:f.rgb}.validate()?;}Ok(())}}
pub struct Playback{venue:Venue,current:Vec<FixtureState>,transition:Option<(Vec<FixtureState>,Cue,f64)>,stopped:bool}
impl Playback{
 pub fn new()->Self{let venue=Venue::demo();let current=venue.fixtures.iter().filter(|f|f.verified&&!f.hazer).map(|f|FixtureState{id:f.id.clone(),intensity:0.0,rgb:[1.0;3]}).collect();Self{venue,current,transition:None,stopped:false}}
 pub fn cue(&mut self,cue:Cue,now:f64)->Result<(),String>{if self.stopped{return Err("Управление остановлено".into());}if !now.is_finite()||now<0.0{return Err("Неверное время".into());}cue.validate()?;for f in &cue.fixtures{self.venue.frame(&[f.id.clone()],f.intensity,f.rgb)?;}self.tick(now)?;self.transition=Some((self.current.clone(),cue,now));Ok(())}
 pub fn tick(&mut self,now:f64)->Result<Vec<u8>,String>{if !now.is_finite()||now<0.0{return Err("Неверное время".into());}if let Some((start,cue,began))=&self.transition{if now<*began{return Err("Время не может идти назад".into());}let t=if cue.seconds==0.0{1.0}else{((now-began)/cue.seconds as f64).min(1.0) as f32};self.current=start.iter().map(|s|{if let Some(target)=cue.fixtures.iter().find(|f|f.id==s.id){let l=Look{intensity:s.intensity,rgb:s.rgb}.fade(Look{intensity:target.intensity,rgb:target.rgb},t).unwrap();FixtureState{id:s.id.clone(),intensity:l.intensity,rgb:l.rgb}}else{s.clone()}}).collect();if t>=1.0{self.transition=None;}}
 let mut frame=vec![0;512];for f in &self.current{let rendered=self.venue.frame(&[f.id.clone()],f.intensity,f.rgb)?;let address=self.venue.fixtures.iter().find(|v|v.id==f.id).unwrap().address-1;frame[address..address+4].copy_from_slice(&rendered[address..address+4]);}Ok(frame)}
 pub fn snapshot(&self,name:String,seconds:f32)->Cue{Cue{name,seconds,fixtures:self.current.clone()}}
 pub fn pause(&mut self,now:f64)->Result<(),String>{self.tick(now)?;self.transition=None;Ok(())}
 pub fn stop(&mut self){self.transition=None;self.stopped=true;}
}
impl Default for Playback{fn default()->Self{Self::new()}}
