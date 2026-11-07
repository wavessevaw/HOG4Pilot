//! Conservative CSV import: offset,function,neutral,min,max. Imported profiles are never verified.
use crate::fixture::{Profile,Channel,Function};
pub fn dmx_csv(id:&str,source:&str,text:&str)->Result<Profile,String>{
 if id.trim().is_empty() || source.trim().is_empty(){return Err("Нужны название и источник таблицы".into());}
 let mut channels=Vec::new();
 for (row,line) in text.lines().enumerate(){if row==0 && line.trim()=="offset,function,neutral,min,max"{continue;}if line.trim().is_empty(){continue;}
 let cells:Vec<_>=line.split(',').map(str::trim).collect();
 if cells.len()!=5{return Err(format!("Строка {}: требуется пять столбцов",row+1));}
 let offset=cells[0].parse::<usize>().map_err(|_|"Некорректный номер канала")?;
 if offset!=channels.len()+1 || offset>512{return Err("Каналы должны идти подряд от 1 до 512".into());}
 let function=match cells[1]{"intensity"=>Function::Intensity,"red"=>Function::Red,"green"=>Function::Green,"blue"=>Function::Blue,"locked"=>Function::Locked,_=>return Err(format!("Строка {}: функция пока не поддерживается",row+1))};
 let parse=|s:&str|s.parse::<u8>().map_err(|_|"Значение должно быть от 0 до 255".to_string());
 let neutral=parse(cells[2])?;let min=parse(cells[3])?;let max=parse(cells[4])?;
 if min>max || !(min..=max).contains(&neutral){return Err("Нейтральное значение вне диапазона".into());}
 channels.push(Channel{function,neutral,min,max});
 }
 if channels.is_empty(){return Err("Таблица пуста".into());}
 Ok(Profile{id:id.into(),version:1,source:source.into(),verified:false,channels})
}
