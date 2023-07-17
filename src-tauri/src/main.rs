/*
TODO:
* [] Create Races Database from csv
*/


// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
mod database;

fn main() -> Result<(), Box<dyn Error>> {
  database::create_tables()?;
  database::populate_tables_via_csv()?;

  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![greet])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");

  Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Here is your string from the front end: {}", name)
}


#[tauri::command]
fn get_races(year: &str) -> Vec<database::Races> {
  let races: Vec<database::Races> = database::get_races(String::from("2023")).expect("ERROR: Unable to get races");
  println!("{:?}",races);
  return races
}