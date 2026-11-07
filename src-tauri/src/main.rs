#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::Mutex;
use hog4pilot::{project::{Store,Project},safety::SafetyEngine,fixture::virtual_rgb,lighting::Look};
use tauri::Manager;
struct State {stopped:std::sync::atomic::AtomicBool,store:Mutex<Store>,engine:Mutex<SafetyEngine>}
#[tauri::command]
fn save(project:Project,state:tauri::State<State>)->Result<(),String>{state.store.lock().map_err(|_|"Хранилище недоступно")?.save(&project)}
#[tauri::command]
fn load(state:tauri::State<State>)->Result<Option<Project>,String>{state.store.lock().map_err(|_|"Хранилище недоступно")?.load()}
#[tauri::command]
fn preview(intensity:f32,rgb:[f32;3],state:tauri::State<State>)->Result<Vec<u8>,String>{Ok(state.engine.lock().map_err(|_|"Движок недоступен")?.render("desktop",Look{intensity,rgb})?.values().to_vec())}
#[tauri::command]
fn stop(state:tauri::State<State>)->Result<(),String>{state.stopped.store(true,std::sync::atomic::Ordering::SeqCst);state.engine.lock().map_err(|_|"Движок недоступен")?.stop();Ok(())}
#[tauri::command]
fn venue_load(state:tauri::State<State>)->Result<hog4pilot::venue::Venue,String>{Ok(state.store.lock().map_err(|_|"Хранилище недоступно")?.load_venue()?.unwrap_or_else(hog4pilot::venue::Venue::demo))}
#[tauri::command]
fn venue_action(action:String,field:Option<String>,value:Option<String>,state:tauri::State<State>)->Result<hog4pilot::venue::Venue,String>{let store=state.store.lock().map_err(|_|"Хранилище недоступно")?;let mut v=store.load_venue()?.unwrap_or_else(hog4pilot::venue::Venue::demo);match action.as_str(){"answer"=>v.answer(field.as_deref().ok_or("Нет этапа")?,value)?,"next"=>v.next()?,_=>return Err("Неизвестное действие".into())};store.save_venue(&v)?;Ok(v)}
#[tauri::command]
fn venue_frame(identify:bool,state:tauri::State<State>)->Result<Vec<u8>,String>{if state.stopped.load(std::sync::atomic::Ordering::SeqCst){return Err("Управление остановлено".into());}let store=state.store.lock().map_err(|_|"Хранилище недоступно")?;let v=store.load_venue()?.unwrap_or_else(hog4pilot::venue::Venue::demo);if identify{v.identify()}else{let ids=v.groups().into_iter().flat_map(|g|g.fixtures).collect::<Vec<_>>();v.frame(&ids,0.6,[1.0,0.65,0.3])}}
#[tauri::command]
fn venue_import(csv:String,state:tauri::State<State>)->Result<hog4pilot::venue::Venue,String>{let v=hog4pilot::venue::Venue::import_patch(&csv)?;state.store.lock().map_err(|_|"Хранилище недоступно")?.save_venue(&v)?;Ok(v)}
fn main(){tauri::Builder::default().setup(|app|{let dir=app.path().app_data_dir()?;std::fs::create_dir_all(&dir)?;let store=Store::open(&dir.join("hog4pilot.sqlite")).map_err(std::io::Error::other)?;let engine=SafetyEngine::new(vec![virtual_rgb("left",1),virtual_rgb("center",5),virtual_rgb("right",9)],Some([0;512]),0,"desktop".into()).map_err(std::io::Error::other)?;app.manage(State{stopped:std::sync::atomic::AtomicBool::new(false),store:Mutex::new(store),engine:Mutex::new(engine)});Ok(())}).invoke_handler(tauri::generate_handler![save,load,preview,stop,venue_load,venue_action,venue_frame,venue_import]).run(tauri::generate_context!()).expect("Не удалось запустить приложение");}
