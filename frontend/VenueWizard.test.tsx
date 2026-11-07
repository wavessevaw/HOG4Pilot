// @vitest-environment jsdom
import {afterEach,expect,test,vi} from 'vitest';
import {cleanup,fireEvent,render,screen,waitFor} from '@testing-library/react';
import {invoke} from '@tauri-apps/api/core';
import {VenueWizard} from './VenueWizard';
vi.mock('@tauri-apps/api/core',()=>({invoke:vi.fn()}));
afterEach(()=>{cleanup();vi.resetAllMocks();});
const venue={name:'Пассивный патч',cursor:0,fixtures:[{id:'Неизвестный',verified:false,hazer:false,mount:'Pending',position:'Pending',role:'Pending',coverage:'Pending'}]};
test('unknown fixture cannot be identified; unknown answer reaches backend',async()=>{vi.mocked(invoke).mockResolvedValue(venue);render(<VenueWizard/>);fireEvent.click(screen.getByText('Открыть / продолжить мастер'));await screen.findByText('Неизвестный — Пассивный режим');expect((screen.getByText('Опознать: включить только в симуляции') as HTMLButtonElement).disabled).toBe(true);fireEvent.click(screen.getByRole('button',{name:'Я не знаю: Место установки'}));await waitFor(()=>expect(invoke).toHaveBeenCalledWith('venue_action',{action:'answer',field:'mount',value:null}));expect(invoke).not.toHaveBeenCalledWith('venue_frame',expect.anything());});
test('backend errors are visible without claiming success',async()=>{vi.mocked(invoke).mockRejectedValue('Хранилище недоступно');render(<VenueWizard/>);fireEvent.click(screen.getByText('Открыть / продолжить мастер'));expect((await screen.findByRole('alert')).textContent).toBe('Хранилище недоступно');});
