#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::Mutex;
use hog4pilot::{project::{Store,Project},safety::SafetyEngine,fixture::virtual_rgb,lighting::Look};
use tauri::Manager;
struct State {store:Mutex<Store>,engine:Mutex<SafetyEngine>}
#[tauri::command]
fn save(project:Project,state:tauri::State<State>)->Result<(),String>{state.store.lock().map_err(|_|"Хранилище недоступно")?.save(&project)}
#[tauri::command]
fn load(state:tauri::State<State>)->Result<Option<Project>,String>{state.store.lock().map_err(|_|"Хранилище недоступно")?.load()}
#[tauri::command]
fn preview(intensity:f32,rgb:[f32;3],state:tauri::State<State>)->Result<Vec<u8>,String>{Ok(state.engine.lock().map_err(|_|"Движок недоступен")?.render("desktop",Look{intensity,rgb})?.values().to_vec())}
#[tauri::command]
fn stop(state:tauri::State<State>)->Result<(),String>{state.engine.lock().map_err(|_|"Движок недоступен")?.stop();Ok(())}
fn main(){tauri::Builder::default().setup(|app|{let dir=app.path().app_data_dir()?;std::fs::create_dir_all(&dir)?;let store=Store::open(&dir.join("hog4pilot.sqlite")).map_err(std::io::Error::other)?;let engine=SafetyEngine::new(vec![virtual_rgb("left",1),virtual_rgb("center",5),virtual_rgb("right",9)],Some([0;512]),0,"desktop".into()).map_err(std::io::Error::other)?;app.manage(State{store:Mutex::new(store),engine:Mutex::new(engine)});Ok(())}).invoke_handler(tauri::generate_handler![save,load,preview,stop]).run(tauri::generate_context!()).expect("Не удалось запустить приложение");}
