import { invoke } from '@tauri-apps/api/tauri';

export async function fetchDriverStandings(): Promise<any> {
  const response = await invoke<any>('get_home_page_driver_standings');
  return response;
}