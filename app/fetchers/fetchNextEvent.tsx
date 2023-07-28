import { invoke } from '@tauri-apps/api/tauri';

export async function fetchNextEvent(): Promise<any> {
  const response = await invoke<any>('get_home_page_next_event');
  return response;
}

// export function delay(ms: number): Promise<void> {
//   return new Promise((resolve) => setTimeout(resolve, ms));
// }

// export async function fetchNextEvent(): Promise<any> {
//   // Add the desired interval (e.g., 1000ms for 1 second) before invoking the API
//   await delay(300);

//   const response = await invoke<any>('get_home_page_next_event');
//   return response;
// }
