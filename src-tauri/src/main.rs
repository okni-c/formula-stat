
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize,Deserialize};
use rand::seq::SliceRandom;


#[derive(Serialize,Clone,Deserialize)]
struct Record {
  id: i8,
  source: String,
}

fn main() {
  let mut people: Vec<Record> = Vec::new();

  let mut i:i8 = 0;
  for _ in 0..100{
    let record: Record = create_record(i);
    people.push(record);
    i += 1;
  }

  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![greet, get_data, send_data, send_all_data])
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

#[tauri::command]
fn send_all_data(people: Vec<Record>) -> Vec<Record> {
  return people;
}

fn create_record(id:i8) -> Record {
  let names: Vec<&str> = vec!["Opal","Marc","Dallas"];
  let name: String = names.choose(&mut rand::thread_rng()).unwrap().to_string();
  let record: Record = Record{
    id:id,
    source:name,
  };
  return record;
}
