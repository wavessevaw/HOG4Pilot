import {VenueWizard} from './VenueWizard';
import {ShowControl} from './ShowControl';
import {useEffect,useState} from 'react';
import {createRoot} from 'react-dom/client';
import {invoke} from '@tauri-apps/api/core';
import './style.css';
type Project={name:string,looks:unknown[]};
function App(){const[page,setPage]=useState(1),[name,setName]=useState('Моё мероприятие'),[message,setMessage]=useState('Симуляция • физический выход выключен');useEffect(()=>{invoke<Project|null>('load').then(p=>{if(p)setName(p.name)}).catch(e=>setMessage(String(e)))},[]);
return <><aside><h1>HOG4<span>Pilot</span></h1><small>СВЕТ БЕЗ СЛОЖНОСТЕЙ</small>{['Подключить площадку','Создать свет','Мои картины','Провести мероприятие'].map((n,i)=><button key={n} className={page===i?'active':''} onClick={()=>setPage(i)}>{i+1}. {n}</button>)}<footer>Локальная работа<br/>Версия для разработки</footer></aside><main><header><span>{message}</span><button className="stop" onClick={async()=>{try{await invoke('stop');setMessage('Управление остановлено до перезапуска приложения')}catch(e){setMessage(String(e))}}}>Остановить управление</button></header><section><label>Проект<input value={name} onChange={e=>setName(e.target.value)}/></label><button onClick={async()=>{try{const old=await invoke<Project|null>('load');await invoke('save',{project:{name,looks:old?.looks??[]}});setMessage('Проект сохранён')}catch(e){setMessage(String(e))}}}>Сохранить проект</button></section><div hidden={page!==0}><VenueWizard/></div><div hidden={page===0}><ShowControl/></div></main></>}
createRoot(document.getElementById('root')!).render(<App/>);
