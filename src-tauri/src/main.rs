#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::Mutex;
use hog4pilot::{project::{Store,Project},safety::SafetyEngine,fixture::virtual_rgb,lighting::Look};
use tauri::Manager;
struct State {playback:std::sync::Arc<Mutex<hog4pilot::playback::Playback>>,clock:std::time::Instant,stopped:std::sync::atomic::AtomicBool,store:Mutex<Store>,engine:Mutex<SafetyEngine>}
#[tauri::command]
fn save(project:Project,state:tauri::State<State>)->Result<(),String>{state.store.lock().map_err(|_|"Хранилище недоступно")?.save(&project)}
#[tauri::command]
fn load(state:tauri::State<State>)->Result<Option<Project>,String>{state.store.lock().map_err(|_|"Хранилище недоступно")?.load()}
#[tauri::command]
fn preview(intensity:f32,rgb:[f32;3],state:tauri::State<State>)->Result<Vec<u8>,String>{Ok(state.engine.lock().map_err(|_|"Движок недоступен")?.render("desktop",Look{intensity,rgb})?.values().to_vec())}
#[tauri::command]
fn stop(state:tauri::State<State>)->Result<(),String>{state.playback.lock().map_err(|_|"Движок недоступен")?.stop();state.stopped.store(true,std::sync::atomic::Ordering::SeqCst);state.engine.lock().map_err(|_|"Движок недоступен")?.stop();Ok(())}
#[tauri::command]
fn venue_load(state:tauri::State<State>)->Result<hog4pilot::venue::Venue,String>{Ok(state.store.lock().map_err(|_|"Хранилище недоступно")?.load_venue()?.unwrap_or_else(hog4pilot::venue::Venue::demo))}
#[tauri::command]
fn venue_action(action:String,field:Option<String>,value:Option<String>,state:tauri::State<State>)->Result<hog4pilot::venue::Venue,String>{let store=state.store.lock().map_err(|_|"Хранилище недоступно")?;let mut v=store.load_venue()?.unwrap_or_else(hog4pilot::venue::Venue::demo);match action.as_str(){"answer"=>v.answer(field.as_deref().ok_or("Нет этапа")?,value)?,"next"=>v.next()?,_=>return Err("Неизвестное действие".into())};store.save_venue(&v)?;Ok(v)}
#[tauri::command]
fn venue_frame(identify:bool,state:tauri::State<State>)->Result<Vec<u8>,String>{if state.stopped.load(std::sync::atomic::Ordering::SeqCst){return Err("Управление остановлено".into());}let store=state.store.lock().map_err(|_|"Хранилище недоступно")?;let v=store.load_venue()?.unwrap_or_else(hog4pilot::venue::Venue::demo);if identify{v.identify()}else{let ids=v.groups().into_iter().flat_map(|g|g.fixtures).collect::<Vec<_>>();v.frame(&ids,0.6,[1.0,0.65,0.3])}}
#[tauri::command]
fn venue_import(csv:String,state:tauri::State<State>)->Result<hog4pilot::venue::Venue,String>{let v=hog4pilot::venue::Venue::import_patch(&csv)?;state.store.lock().map_err(|_|"Хранилище недоступно")?.save_venue(&v)?;Ok(v)}
#[tauri::command]
fn playback_frame(state:tauri::State<State>)->Result<Vec<u8>,String>{state.playback.lock().map_err(|_|"Движок недоступен")?.tick(state.clock.elapsed().as_secs_f64())}
#[tauri::command]
fn playback_cue(cue:hog4pilot::playback::Cue,state:tauri::State<State>)->Result<(),String>{state.playback.lock().map_err(|_|"Движок недоступен")?.cue(cue,state.clock.elapsed().as_secs_f64())}
#[tauri::command]
fn playback_pause(state:tauri::State<State>)->Result<(),String>{state.playback.lock().map_err(|_|"Движок недоступен")?.pause(state.clock.elapsed().as_secs_f64())}
#[tauri::command]
fn cue_save(name:String,seconds:f32,state:tauri::State<State>)->Result<Vec<hog4pilot::playback::Cue>,String>{let mut p=state.playback.lock().map_err(|_|"Движок недоступен")?;p.tick(state.clock.elapsed().as_secs_f64())?;let cue=p.snapshot(name,seconds);let store=state.store.lock().map_err(|_|"Хранилище недоступно")?;let mut cues=store.load_cues()?;cues.push(cue);store.save_cues(&cues)?;Ok(cues)}
#[tauri::command]
fn cue_load(state:tauri::State<State>)->Result<Vec<hog4pilot::playback::Cue>,String>{state.store.lock().map_err(|_|"Хранилище недоступно")?.load_cues()}
#[tauri::command]
fn ufr_import(json:String,state:tauri::State<State>)->Result<(),String>{let p=hog4pilot::recognition::Fingerprint::parse(&json)?;state.store.lock().map_err(|_|"Хранилище недоступно")?.import_fingerprint(&p)}
#[tauri::command]
fn ufr_search(json:String,state:tauri::State<State>)->Result<Vec<hog4pilot::recognition::Comparison>,String>{let p=hog4pilot::recognition::Fingerprint::parse(&json)?;state.store.lock().map_err(|_|"Хранилище недоступно")?.search_fingerprint(&p)}
#[tauri::command]
fn ufr_note(fixture:String,id:String,revision:u32,note:String,state:tauri::State<State>)->Result<(),String>{state.store.lock().map_err(|_|"Хранилище недоступно")?.note_candidate(&fixture,&id,revision,&note)}
fn main(){tauri::Builder::default().setup(|app|{let dir=app.path().app_data_dir()?;std::fs::create_dir_all(&dir)?;let store=Store::open(&dir.join("hog4pilot.sqlite")).map_err(std::io::Error::other)?;let engine=SafetyEngine::new(vec![virtual_rgb("left",1),virtual_rgb("center",5),virtual_rgb("right",9)],Some([0;512]),0,"desktop".into()).map_err(std::io::Error::other)?;let playback=std::sync::Arc::new(Mutex::new(hog4pilot::playback::Playback::new()));let clock=std::time::Instant::now();let worker=playback.clone();std::thread::spawn(move||loop{std::thread::sleep(std::time::Duration::from_millis(25));if let Ok(mut p)=worker.lock(){if p.tick(clock.elapsed().as_secs_f64()).is_err(){p.stop();}}else{break;}});app.manage(State{playback,clock,stopped:std::sync::atomic::AtomicBool::new(false),store:Mutex::new(store),engine:Mutex::new(engine)});Ok(())}).invoke_handler(tauri::generate_handler![save,load,preview,stop,venue_load,venue_action,venue_frame,venue_import,playback_frame,playback_cue,playback_pause,cue_save,cue_load,ufr_import,ufr_search,ufr_note]).run(tauri::generate_context!()).expect("Не удалось запустить приложение");}
