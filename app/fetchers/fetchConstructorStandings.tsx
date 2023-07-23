import { invoke } from '@tauri-apps/api/tauri';

export async function fetchConstructorStandings(): Promise<any> {
  const response = await invoke<any>('get_home_page_constructor_standings');
  return response;
}