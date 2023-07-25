// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use rusqlite::Connection;
use tauri::{State, Manager, AppHandle};
mod state;
use state::{DbState, DatabaseAccess};
mod database;
mod func;

fn main() -> Result<(), Box<dyn Error>> {
  // Comment this out for debugging build
  // database::create_tables()?;
  // database::populate_tables_via_csv()?;

  tauri::Builder::default()
      .manage(DbState {conn: Default::default()})
      .invoke_handler(tauri::generate_handler![
        get_races, get_driver, get_circuit,
        get_home_page_next_event, get_home_page_driver_standings, get_home_page_constructor_standings
        ])
      .setup(|app| {
        let handle: AppHandle = app.handle();
        let db_state: State<DbState> = handle.state();
        let conn = database::connect_to_db(&handle).expect("ERROR: unable to connect to F1.db");
        *db_state.conn.lock().unwrap() = Some(conn);
        Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");

  Ok(())
}

#[tauri::command]
async fn get_races(app_handle: AppHandle, year: String) -> Vec<database::Races> {
  // let races: Vec<database::Races> = database::get_races(year.to_string()).expect("ERROR: Unable to get races");
  let races: Vec<database::Races> = app_handle.conn(|conn| database::get_races(year, conn)).unwrap();
  println!("{:?}",races);
  return races
}

#[tauri::command]
// Given a driver_id, returns a object of driver information
async fn get_driver(app_handle: AppHandle, driver_id: String) -> database::Driver {
  // let driver: database::Driver = database::get_driver(driver_id.to_string()).expect("ERROR: Unable to get driver");
  let driver: database::Driver  = app_handle.conn(|conn| database::get_driver(driver_id, conn)).unwrap();
  println!("{:?}",driver);
  return driver
}

#[tauri::command]
// Given a circuit_id, returns a object of circuit information
async fn get_circuit(app_handle: AppHandle, circuit_id: String) -> database::Circuit {
  // let driver: database::Circuit = database::get_circuit(circuit_id.to_string()).expect("ERROR: Unable to get circuit");
  let driver: database::Circuit  = app_handle.conn(|conn| database::get_circuit(circuit_id, conn)).unwrap();
  println!("{:?}",driver);
  return driver
}

#[tauri::command]
// Returns all information for the next race event for the homepage
async fn get_home_page_next_event(app_handle: AppHandle) -> database::NextEvent {
  // let next_event: database::NextEvent = database::home_page_next_event().expect("ERROR: Unable to get Next Race for Homepage");
  let next_event: database::NextEvent  = app_handle.conn(|conn| database::home_page_next_event(conn)).unwrap();
  println!("{:?}",next_event);
  return next_event
}

#[tauri::command]
// Returns a Vector of Driver Standing Objects Ascending based on position for the homepage
async fn get_home_page_driver_standings(app_handle: AppHandle) -> Vec<database::DriverStanding> {
  // let driver_standings: Vec<database::DriverStanding> = database::home_page_driver_standings().expect("ERROR: Unable to get circuit");
  let driver_standings: Vec<database::DriverStanding>  = app_handle.conn(|conn| database::home_page_driver_standings(conn)).unwrap();
  println!("{:?}",driver_standings);
  return driver_standings
}

#[tauri::command]
// Returns a Vector of Constructor Standing Objects Ascending based on position for the homepage
async fn get_home_page_constructor_standings(app_handle: AppHandle) -> Vec<database::ConstructorStanding> {
  // let constructor_standings: Vec<database::ConstructorStanding> = database::home_page_constructor_standings().expect("ERROR: Unable to get circuit");
  let constructor_standings: Vec<database::ConstructorStanding>  = app_handle.conn(|conn| database::home_page_constructor_standings(conn)).unwrap();
  println!("{:?}",constructor_standings);
  return constructor_standings
}