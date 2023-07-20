// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
mod database;

fn main() -> Result<(), Box<dyn Error>> {
  database::create_tables()?;
  database::populate_tables_via_csv()?;

  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![
        greet, get_races, get_driver, get_circuit,
        get_home_page_next_event, get_home_page_driver_standings, get_home_page_constructor_standings
        ])
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
  let races: Vec<database::Races> = database::get_races(year.to_string()).expect("ERROR: Unable to get races");
  println!("{:?}",races);
  return races
}

#[tauri::command]
// Given a driver_id, returns a object of driver information
fn get_driver(driver_id: &str) -> database::Driver {
  let driver: database::Driver = database::get_driver(driver_id.to_string()).expect("ERROR: Unable to get driver");
  println!("{:?}",driver);
  return driver
}

#[tauri::command]
// Given a circuit_id, returns a object of circuit information
fn get_circuit(circuit_id: &str) -> database::Circuit {
  let driver: database::Circuit = database::get_circuit(circuit_id.to_string()).expect("ERROR: Unable to get circuit");
  println!("{:?}",driver);
  return driver
}

#[tauri::command]
// Returns all information for the next race event for the homepage
fn get_home_page_next_event() -> database::Races {
  let next_event: database::Races = database::home_page_next_event().expect("ERROR: Unable to get Next Race for Homepage");
  println!("{:?}",next_event);
  return next_event
}

#[tauri::command]
// Returns a Vector of Driver Standing Objects Ascending based on position for the homepage
fn get_home_page_driver_standings() -> Vec<database::DriverStanding> {
  let driver_standings: Vec<database::DriverStanding> = database::home_page_driver_standings().expect("ERROR: Unable to get circuit");
  println!("{:?}",driver_standings);
  return driver_standings
}

#[tauri::command]
// Returns a Vector of Constructor Standing Objects Ascending based on position for the homepage
fn get_home_page_constructor_standings() -> Vec<database::ConstructorStanding> {
  let constructor_standings: Vec<database::ConstructorStanding> = database::home_page_constructor_standings().expect("ERROR: Unable to get circuit");
  println!("{:?}",constructor_standings);
  return constructor_standings
}