import { invoke } from '@tauri-apps/api/tauri';

export async function fetchRaces(year: string): Promise<any> {
  const response = await invoke<any>('get_races', { year: year });
  return response;
}