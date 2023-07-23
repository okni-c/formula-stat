import { invoke } from '@tauri-apps/api/tauri';

export async function fetchNextEvent(): Promise<any> {
  const response = await invoke<any>('get_home_page_next_event');
  return response;
}