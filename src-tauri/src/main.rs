
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
#[derive(Serialize)]
struct Record {
  id: i8,
  source: String,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_data, send_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Here is your string from the front end: {}", name)
}

#[tauri::command]
fn get_data() -> String {
  "This data was sent from the back end only.".into()
}

#[tauri::command]
fn send_data() -> Record {
  let r1 = Record {
    id: 1,
    source: String::from("Opal"),
  };
  return r1;
}
