// @vitest-environment jsdom
import {afterEach,expect,test,vi} from 'vitest';
import {cleanup,fireEvent,render,screen} from '@testing-library/react';
import {invoke} from '@tauri-apps/api/core';
import {Recognition} from './Recognition';
vi.mock('@tauri-apps/api/core',()=>({invoke:vi.fn()}));
afterEach(()=>{cleanup();vi.resetAllMocks();});
test('incompatible results explain differences and cannot be saved as candidate',async()=>{vi.mocked(invoke).mockResolvedValue([{candidate:'OEM',revision:1,verdict:'incompatible',differences:['Канал 1: Reset вместо яркости'],missing:[],hazardous:true}]);render(<Recognition fixture="A"/>);fireEvent.change(screen.getByLabelText('Описание профиля JSON'),{target:{value:'{}'}});fireEvent.click(screen.getByText('Найти совпадения'));await screen.findByText('Канал 1: Reset вместо яркости');expect(screen.queryByText('Сохранить кандидата без разрешения управления')).toBeNull();expect(invoke).toHaveBeenCalledWith('ufr_search',{json:'{}'});});
test('unknown can proceed without network or output calls',()=>{render(<Recognition fixture="A"/>);fireEvent.click(screen.getByText('Я не знаю'));expect(screen.getByRole('status').textContent).toContain('остаётся пассивным');expect(invoke).not.toHaveBeenCalled();});
