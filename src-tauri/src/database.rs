use rusqlite::{Connection, params};
use std::{error::Error, vec};
use std::fs::File;
use csv::ReaderBuilder;
use serde::Serialize;
use chrono::{DateTime, Local, TimeZone, NaiveDate, NaiveTime, ParseError, Timelike, NaiveDateTime, Duration};

use crate::func::{get_country_code_country, get_country_code_nationality, set_to_null_if_n, to_datetime, parse_datetime};


#[derive(Debug)]
#[derive(Serialize)]
pub struct Races {
    race_id: i64,
    year: i64,
    round: i64,
    circuit_id: i64,
    name: String,
    date: String,
    time: String,
    url: String,
    fp1_date: String,
    fp1_time: String,
    fp2_date: String,
    fp2_time: String,
    fp3_date: String,
    fp3_time: String,
    quai_date: String,
    quali_time: String,
    sprint_date: String,
    sprint_time: String,
    country: String,
    country_code: String,
}

#[derive(Debug)]
#[derive(Serialize)]
pub struct Driver {
    driver_id: i64,
    driver_ref: String,
    number: i64,
    code: String,
    forename: String,
    surename: String,
    dob: String,
    nationality: String,
    url: String,
    country_code: String,
}

#[derive(Debug)]
#[derive(Serialize)]
pub struct DriverStanding {
    driver_id: i64,
    points: i64,
    position: i64,
    wins: i64,
    forename: String,
    surename: String,
    nationality: String,
    country_code: String,
}

#[derive(Debug)]
#[derive(Serialize)]
pub struct ConstructorStanding {
    constructor_id: i64,
    points: i64,
    position: i64,
    wins: i64,
    name: String,
    nationality: String,
    country_code: String,
}

#[derive(Debug)]
#[derive(Serialize)]
pub struct Circuit {
    circuit_id: i64,
    circuit_ref: String,
    name: String,
    location: String,
    country: String,
    lat: String,
    lng: String,
    alt: String,
    url: String
}

/*
Establish a Connection to the f1_db and return that connection
*/
pub fn connect_to_db() -> Result<Connection, Box<dyn Error>> {
    let conn: Connection = Connection::open("f1.db").expect("ERROR: Unable to connect to f1.db");
    Ok(conn)
}

/*
Creates all required tables if they dont exists
*/
pub fn create_tables() -> Result<(), Box<dyn Error>> {
    let conn: Connection = connect_to_db()?;
    println!("Initalizing F1.db");
    // Race Table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS races (
            raceId INTEGER PRIMARY KEY,
            year INTEGER DEFAULT '0',
            round INTEGER DEFAULT '0',
            circuitId INTEGER DEFAULT '0',
            name TEXT DEFAULT '',
            date TEXT DEFAULT '0000-00-00',
            time TEXT DEFAULT NULL,
            url TEXT DEFAULT NULL,
            fp1_date TEXT DEFAULT NULL,
            fp1_time TEXT DEFAULT NULL,
            fp2_date TEXT DEFAULT NULL,
            fp2_time TEXT DEFAULT NULL,
            fp3_date TEXT DEFAULT NULL,
            fp3_time TEXT DEFAULT NULL,
            quali_date TEXT DEFAULT NULL,
            quali_time TEXT DEFAULT NULL,
            sprint_date TEXT DEFAULT NULL,
            sprint_time TEX DEFAULT NULLT
        )", [],
    )?;
    println!("TABLE CREATED: races");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS lapTimes (
            raceId INTEGER NOT NULL,
            driverId INTEGER NOT NULL,
            lap INTEGER NOT NULL,
            position INTEGER DEFAULT NULL,
            time TEXT DEFAULT NULL,
            milliseconds INTEGER DEFAULT NULL,
            PRIMARY KEY (raceId, driverId, lap)
        )", [],
    )?;
    println!("TABLE CREATED: lapTimes");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS qualifying (
            qualifyId INTEGER NOT NULL,
            raceId INTEGER DEFAULT '0',
            driverId INTEGER DEFAULT '0',
            constructorId INTEGER DEFAULT '0',
            number INTEGER DEFAULT '0',
            position INTEGER DEFAULT NULL,
            q1 TEXT DEFAULT NULL,
            q2 TEXT DEFAULT NULL,
            q3 TEXT DEFAULT NULL,
            PRIMARY KEY (qualifyId)
        )", [],
    )?;
    println!("TABLE CREATED: qualifying");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS results (
            resultId INTEGER PRIMARY KEY,
            raceId INTEGER DEFAULT '0',
            driverId INTEGER DEFAULT '0',
            constructorId INTEGER DEFAULT '0',
            number INTEGER DEFAULT '',
            grid INTEGER DEFAULT '0',
            position INTEGER DEFAULT NULL,
            positionText TEXT DEFAULT '',
            positionOrder INTEGER DEFAULT '0',
            points NUMBER DEFAULT '0',
            laps INTEGER DEFAULT '0',
            time TEXT DEFAULT NULL,
            milliseconds INTEGER DEFAULT NULL,
            fastestLap INTEGER DEFAULT NULL,
            rank INTEGER DEFAULT '0',
            fastestLapTime TEXT DEFAULT NULL,
            fastestLapSpeed TEXT DEFAULT NULL,
            statusId INTEGER DEFAULT '0'
        )", [],
    )?;
    println!("TABLE CREATED: results");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS sprintResults (
            sprintResultId INTEGER NOT NULL,
            raceId INTEGER DEFAULT '0',
            driverId INTEGER DEFAULT '0',
            constructorId INTEGER DEFAULT '0',
            number INTEGER DEFAULT '',
            grid INTEGER DEFAULT '0',
            position INTEGER DEFAULT NULL,
            positionText TEXT DEFAULT '',
            positionOrder INTEGER DEFAULT '0',
            points NUMBER DEFAULT '0',
            laps INTEGER DEFAULT '0',
            time TEXT DEFAULT NULL,
            milliseconds INTEGER DEFAULT NULL,
            fastestLap INTEGER DEFAULT NULL,
            fastestLapTime TEXT DEFAULT NULL,
            statusId INTEGER DEFAULT '0',
            PRIMARY KEY (sprintResultId)
        )", [],
    )?;
    println!("TABLE CREATED: sprintResults");
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS drivers (
            driverId INTEGER PRIMARY KEY,
            driverRef TEXT DEFAULT '',
            number INTEGER DEFAULT NULL,
            code TEXT DEFAULT NULL,
            forename TEXT DEFAULT '',
            surename TEXT DEFAULT '',
            dob TEXT DEFAULT NULL,
            nationality TEXT DEFAULT NULL,
            url TEXT DEFAULT NULL
        )", [],
    )?;
    println!("TABLE CREATED: drivers");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS driverStandings (
            driverStandingsId INTEGER PRIMARY KEY,
            raceId INTEGER DEFAULT '0',
            driverId INTEGER DEFAULT '0',
            points NUMBER DEFAULT '0',
            position INTEGER DEFAULT '0',
            positionText TEXT DEFAULT '',
            wins INTEGER DEFAULT '0'
        )", [],
    )?;
    println!("TABLE CREATED: driverStandings");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS circuits (
            circuitId INTEGER PRIMARY KEY,
            circuitRef TEXT DEFAULT '',
            name TEXT DEFAULT '',
            location TEXT DEFAULT NULL,
            country TEXT DEFAULT NULL,
            lat TEXT DEFAULT '',
            lng TEXT DEFAULT '',
            alt TEXT DEFAULT NULL,
            url TEXT DEFAULT NULL
        )", [],
    )?;
    println!("TABLE CREATED: circuits");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS constructors (
            constructorId INTEGER PRIMARY KEY,
            constructorRef TEXT DEFAULT '',
            name TEXT DEFAULT '',
            nationality TEXT DEFAULT NULL,
            url TEXT DEFAULT NULL
        )", [],
    )?;
    println!("TABLE CREATED: constructors");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS constructorStandings (
            constructorStandingsId INTEGER PRIMARY KEY,
            raceId INTEGER DEFAULT '0',
            constructorId INTEGER DEFAULT '0',
            points NUMBER DEFAULT '0',
            position INTEGER DEFAULT NULL,
            positionText TEXT DEFAULT NULL,
            wins INTEGER DEFAULT '0'
        )", [],
    )?;
    println!("TABLE CREATED: constructorStandings");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS constructorResults (
            constructorResultsId INTEGER PRIMARY KEY,
            raceId INTEGER DEFAULT '0',
            constructorId INTEGER DEFAULT '0',
            points NUMBER DEFAULT NULL,
            status TEXT DEFAULT NULL
        )", [],
    )?;
    println!("TABLE CREATED: constructorResults");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS status (
            statusId INTEGER,
            status TEXT DEFAULT '',
            PRIMARY KEY (statusId)
        )", [],
    )?;
    println!("TABLE CREATED: status");

    Ok(())
}

/*
Populating all tables with thier respective CSV if they dont match in size; Else they are already populated
*/
pub fn populate_tables_via_csv() -> Result<(), Box<dyn Error>>{
    let conn: Connection = connect_to_db()?;
    // Races
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM races", [], |row| row.get(0))?;
    races_csv(&conn, sql_rows, String::from("data/races.csv"));

    // Lap Times
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM lapTimes", [], |row| row.get(0))?;
    lap_times_csv(&conn, sql_rows, String::from("data/lap_times.csv"));

    // Qualifying
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM qualifying", [], |row| row.get(0))?;
    qualifying_csv(&conn, sql_rows, String::from("data/qualifying.csv"));

    // Results
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM results", [], |row| row.get(0))?;
    results_csv(&conn, sql_rows, String::from("data/results.csv"));

    // Sprint Results
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM sprintResults", [], |row| row.get(0))?;
    sprint_results_csv(&conn, sql_rows, String::from("data/sprint_results.csv"));

    // Drivers
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM drivers", [], |row| row.get(0))?;
    drivers_csv(&conn, sql_rows, String::from("data/drivers.csv"));

    // Driver Standings
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM driverStandings", [], |row| row.get(0))?;
    driver_standings_csv(&conn, sql_rows, String::from("data/driver_standings.csv"));

    // Constructors
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM constructors", [], |row| row.get(0))?;
    constructors_csv(&conn, sql_rows, String::from("data/constructors.csv"));

    // Constructor Standings
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM constructorStandings", [], |row| row.get(0))?;
    constructor_standings_csv(&conn, sql_rows, String::from("data/constructor_standings.csv"));

    // Constructor Results
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM constructorResults", [], |row| row.get(0))?;
    constructor_results_csv(&conn, sql_rows, String::from("data/constructor_results.csv"));

    // Circuits
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM circuits", [], |row| row.get(0))?;
    circuits_csv(&conn, sql_rows, String::from("data/circuits.csv"));

    // Status
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM status", [], |row| row.get(0))?;
    status_csv(&conn, sql_rows, String::from("data/status.csv"));

    Ok(())
}

/*
Reads the races.csv and populates the races table with all rows
*/
fn races_csv(conn: &Connection, sql_rows: i64, filepath: String) {
    let file = File::open(filepath.clone()).expect("ERROR: Unable to access CSV");
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
    let csv_rows: usize = reader.records().count();
    if sql_rows != csv_rows as i64 {
        let file = File::open(filepath).expect("ERROR: Unable to access CSV");
        let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
        for row in reader.records() {
            let record = row.expect("ERROR: Unable to access row");
            let race_id = &record[0];
            let year = &record[1];
            let round = &record[2];
            let circuit_id = &record[3];
            let name = &record[4];
            let date = &record[5];
            let time = &record[6];
            let url = &record[7];
            let fp1_date = &record[8];
            let fp1_time = &record[9];
            let fp2_date = &record[10];
            let fp2_time = &record[11];
            let fp3_date = &record[12];
            let fp3_time = &record[13];
            let quali_date = &record[14];
            let quali_time = &record[15];
            let sprint_date = &record[16];
            let sprint_time = &record[17];

            conn.execute(
                "INSERT INTO races (
                    raceId,
                    year,
                    round,
                    circuitId,
                    name,
                    date,
                    time,
                    url,
                    fp1_date,
                    fp1_time,
                    fp2_date,
                    fp2_time,
                    fp3_date,
                    fp3_time,
                    quali_date,
                    quali_time,
                    sprint_date,
                    sprint_time
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)", 
                params![
                    race_id,year,round,circuit_id,name,date,time,url,
                    fp1_date,fp1_time,fp2_date,fp2_time,fp3_date,fp3_time,
                    quali_date,quali_time,sprint_date,sprint_time,
                    ],
                ).expect("ERROR: Unable to Insert Race Row");
        }
        println!("races TABLE POPULATED VIA CSV");
    } else {
        println!("races TABLE ALREADY UP-TO-DATE");
    }

}

/*
Reads the lap_times.csv and populates the lapTimes table with all rows
*/
fn lap_times_csv(conn: &Connection, sql_rows: i64, filepath: String) {
    let file = File::open(filepath.clone()).expect("ERROR: Unable to access CSV");
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
    let csv_rows: usize = reader.records().count();
    if sql_rows != csv_rows as i64 {
        let file = File::open(filepath).expect("ERROR: Unable to access CSV");
        let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
        for row in reader.records() {
            let record = row.expect("ERROR: Unable to access row");
            let race_id = &record[0];
            let driver_id = &record[1];
            let lap = &record[2];
            let position = &record[3];
            let time = &record[4];
            let milliseconds = &record[5];

            conn.execute(
                "INSERT INTO lapTimes (
                    raceId,
                    driverId,
                    lap,
                    position,
                    time,
                    milliseconds
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", 
                params![
                    race_id,driver_id,lap,position,time,milliseconds,
                    ],
                ).expect("ERROR: Unable to Insert lapTimes Row");
        }
        println!("lapTimes TABLE POPULATED VIA CSV");
    } else {
        println!("lapTimes TABLE ALREADY UP-TO-DATE");
    }

}

/*
Reads the qualifying.csv and populates the qualifying table with all rows
*/
fn qualifying_csv(conn: &Connection, sql_rows: i64, filepath: String) {
    let file = File::open(filepath.clone()).expect("ERROR: Unable to access CSV");
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
    let csv_rows: usize = reader.records().count();
    if sql_rows != csv_rows as i64 {
        let file = File::open(filepath).expect("ERROR: Unable to access CSV");
        let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
        for row in reader.records() {
            let record = row.expect("ERROR: Unable to access row");
            let qualify_id = &record[0];
            let race_id = &record[1];
            let driver_id = &record[2];
            let constructor_id = &record[3];
            let number = &record[4];
            let position = &record[5];
            let q1 = &record[6];
            let q2 = &record[7];
            let q3 = &record[8];

            conn.execute(
                "INSERT INTO qualifying (
                    qualifyId,
                    raceId,
                    driverId,
                    constructorId,
                    number,
                    position,
                    q1,
                    q2,
                    q3
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)", 
                params![
                    qualify_id,race_id,driver_id,constructor_id,number,position,q1,q2,q3,
                    ],
                ).expect("ERROR: Unable to Insert qualifying Row");
        }
        println!("qualifying TABLE POPULATED VIA CSV");
    } else {
        println!("qualifying TABLE ALREADY UP-TO-DATE");
    }

}

/*
Reads the results.csv and populates the results table with all rows
*/
fn results_csv(conn: &Connection, sql_rows: i64, filepath: String) {
    let file = File::open(filepath.clone()).expect("ERROR: Unable to access CSV");
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
    let csv_rows: usize = reader.records().count();
    if sql_rows != csv_rows as i64 {
        let file = File::open(filepath).expect("ERROR: Unable to access CSV");
        let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
        for row in reader.records() {
            let record = row.expect("ERROR: Unable to access row");
            let result_id = &record[0];
            let race_id = &record[1];
            let driver_id = &record[2];
            let constructor_id = &record[3];
            let number = &record[4];
            let grid = &record[5];
            let position = &record[6];
            let position_text = &record[7];
            let position_order = &record[8];
            let points = &record[9];
            let laps = &record[10];
            let time = &record[11];
            let milliseconds = &record[12];
            let fastest_lap = &record[13];
            let rank = &record[14];
            let fastest_lap_time = &record[15];
            let fastest_lap_speed = &record[16];
            let status_id = &record[17];

            conn.execute(
                "INSERT INTO results (
                    resultId,
                    raceId,
                    driverId,
                    constructorId,
                    number,
                    grid,
                    position,
                    positionText,
                    positionOrder,
                    points,
                    laps,
                    time,
                    milliseconds,
                    fastestLap,
                    rank,
                    fastestLapTime,
                    fastestLapSpeed,
                    statusId
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)", 
                params![
                    result_id,race_id,driver_id,constructor_id,number,grid,position,position_text,
                    position_order,points,laps,time,milliseconds,fastest_lap,
                    rank,fastest_lap_time,fastest_lap_speed,status_id,
                    ],
                ).expect("ERROR: Unable to Insert results Row");
        }
        println!("results TABLE POPULATED VIA CSV");
    } else {
        println!("results TABLE ALREADY UP-TO-DATE");
    }

}

/*
Reads the sprint_results.csv and populates the sprintResults table with all rows
*/
fn sprint_results_csv(conn: &Connection, sql_rows: i64, filepath: String) {
    let file = File::open(filepath.clone()).expect("ERROR: Unable to access CSV");
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
    let csv_rows: usize = reader.records().count();
    if sql_rows != csv_rows as i64 {
        let file = File::open(filepath).expect("ERROR: Unable to access CSV");
        let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
        for row in reader.records() {
            let record = row.expect("ERROR: Unable to access row");
            let sprint_result_id = &record[0];
            let race_id = &record[1];
            let driver_id = &record[2];
            let constructor_id = &record[3];
            let number = &record[4];
            let grid = &record[5];
            let position = &record[6];
            let position_text = &record[7];
            let position_order = &record[8];
            let points = &record[9];
            let laps = &record[10];
            let time = &record[11];
            let milliseconds = &record[12];
            let fastest_lap = &record[13];
            let fastest_lap_time = &record[14];
            let status_id = &record[15];

            conn.execute(
                "INSERT INTO sprintResults (
                    sprintResultId,
                    raceId,
                    driverId,
                    constructorId,
                    number,
                    grid,
                    position,
                    positionText,
                    positionOrder,
                    points,
                    laps,
                    time,
                    milliseconds,
                    fastestLap,
                    fastestLapTime,
                    statusId
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)", 
                params![
                    sprint_result_id,race_id,driver_id,constructor_id,number,grid,position,position_text,
                    position_order,points,laps,time,milliseconds,fastest_lap,
                    fastest_lap_time,status_id,
                    ],
                ).expect("ERROR: Unable to Insert sprintResults Row");
        }
        println!("sprintResults TABLE POPULATED VIA CSV");
    } else {
        println!("sprintResults TABLE ALREADY UP-TO-DATE");
    }

}

/*
Reads the drivers.csv and populates the drivers table with all rows
*/
fn drivers_csv(conn: &Connection, sql_rows: i64, filepath: String) {
    let file = File::open(filepath.clone()).expect("ERROR: Unable to access CSV");
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
    let csv_rows: usize = reader.records().count();
    if sql_rows != csv_rows as i64 {
        let file = File::open(filepath).expect("ERROR: Unable to access CSV");
        let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
        for row in reader.records() {
            let record = row.expect("ERROR: Unable to access row");
            let driver_id = &record[0];
            let driver_ref = &record[1];
            let number = &record[2];
            let code = &record[3];
            let forename = &record[4];
            let surename = &record[5];
            let dob = &record[6];
            let nationality = &record[7];
            let url = &record[8];

            conn.execute(
                "INSERT INTO drivers (
                    driverId,
                    driverRef,
                    number,
                    code,
                    forename,
                    surename,
                    dob,
                    nationality,
                    url
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)", 
                params![
                    driver_id,driver_ref,number,code,forename,surename,dob,nationality,url,
                    ],
                ).expect("ERROR: Unable to Insert Driver Row");
        }
        println!("drivers TABLE POPULATED VIA CSV");
    } else {
        println!("drivers TABLE ALREADY UP-TO-DATE");
    }

}

/*
Reads the driver_standings.csv and populates the driver_standings table with all rows
*/
fn driver_standings_csv(conn: &Connection, sql_rows: i64, filepath: String) {
    let file = File::open(filepath.clone()).expect("ERROR: Unable to access CSV");
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
    let csv_rows: usize = reader.records().count();
    if sql_rows != csv_rows as i64 {
        let file = File::open(filepath).expect("ERROR: Unable to access CSV");
        let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
        for row in reader.records() {
            let record = row.expect("ERROR: Unable to access row");
            let driver_standings_id = &record[0];
            let race_id = &record[1];
            let driver_id = &record[2];
            let points = &record[3];
            let position = &record[4];
            let position_text = &record[5];
            let wins = &record[6];

            conn.execute(
                "INSERT INTO driverStandings (
                    driverStandingsId,
                    raceId,
                    driverId,
                    points,
                    position,
                    positionText,
                    wins
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", 
                params![
                    driver_standings_id,race_id,driver_id,points,position,position_text,wins,
                    ],
                ).expect("ERROR: Unable to Insert Driver Standings Row");
        }
        println!("driverStandings TABLE POPULATED VIA CSV");
    } else {
        println!("driverStandings TABLE ALREADY UP-TO-DATE");
    }

}

/*
Reads the constructors.csv and populates the constructors table with all rows
*/
fn constructors_csv(conn: &Connection, sql_rows: i64, filepath: String) {
    let file = File::open(filepath.clone()).expect("ERROR: Unable to access CSV");
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
    let csv_rows: usize = reader.records().count();
    if sql_rows != csv_rows as i64 {
        let file = File::open(filepath).expect("ERROR: Unable to access CSV");
        let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
        for row in reader.records() {
            let record = row.expect("ERROR: Unable to access row");
            let constructor_id = &record[0];
            let constructor_ref = &record[1];
            let name = &record[2];
            let nationality = &record[3];
            let url = &record[4];

            conn.execute(
                "INSERT INTO constructors (
                    constructorId,
                    constructorRef,
                    name,
                    nationality,
                    url
                ) VALUES (?1, ?2, ?3, ?4, ?5)", 
                params![
                    constructor_id,constructor_ref,name,nationality,url,
                    ],
                ).expect("ERROR: Unable to Insert Driver Standings Row");
        }
        println!("constructors TABLE POPULATED VIA CSV");
    } else {
        println!("constructors TABLE ALREADY UP-TO-DATE");
    }

}

/*
Reads the constructor_standings.csv and populates the constructorStandings_table with all rows
*/
fn constructor_standings_csv(conn: &Connection, sql_rows: i64, filepath: String) {
    let file = File::open(filepath.clone()).expect("ERROR: Unable to access CSV");
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
    let csv_rows: usize = reader.records().count();
    if sql_rows != csv_rows as i64 {
        let file = File::open(filepath).expect("ERROR: Unable to access CSV");
        let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
        for row in reader.records() {
            let record = row.expect("ERROR: Unable to access row");
            let constructor_standings_id = &record[0];
            let race_id = &record[1];
            let constructor_id = &record[2];
            let points = &record[3];
            let position = &record[4];
            let position_text = &record[5];
            let wins = &record[6];

            conn.execute(
                "INSERT INTO constructorStandings (
                    constructorStandingsId,
                    raceId,
                    constructorId,
                    points,
                    position,
                    positionText,
                    wins
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", 
                params![
                    constructor_standings_id,race_id,constructor_id,points,position,position_text,wins,
                    ],
                ).expect("ERROR: Unable to Insert Constructor Standings Row");
        }
        println!("constructorStandings TABLE POPULATED VIA CSV");
    } else {
        println!("constructorStandings TABLE ALREADY UP-TO-DATE");
    }

}

/*
Reads the constructor_results.csv and populates the constructorResults table with all rows
*/
fn constructor_results_csv(conn: &Connection, sql_rows: i64, filepath: String) {
    let file = File::open(filepath.clone()).expect("ERROR: Unable to access CSV");
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
    let csv_rows: usize = reader.records().count();
    if sql_rows != csv_rows as i64 {
        let file = File::open(filepath).expect("ERROR: Unable to access CSV");
        let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
        for row in reader.records() {
            let record = row.expect("ERROR: Unable to access row");
            let constructor_results_id = &record[0];
            let race_id = &record[1];
            let constructor_id = &record[2];
            let points = &record[3];
            let status = &record[4];

            conn.execute(
                "INSERT INTO constructorResults (
                    constructorResultsId,
                    raceId,
                    constructorId,
                    points,
                    status
                ) VALUES (?1, ?2, ?3, ?4, ?5)", 
                params![
                    constructor_results_id,race_id,constructor_id,points,status,
                    ],
                ).expect("ERROR: Unable to Insert Constructor Results Row");
        }
        println!("constructorResults TABLE POPULATED VIA CSV");
    } else {
        println!("constructorResults TABLE ALREADY UP-TO-DATE");
    }

}

/*
Reads the circuits.csv and populates the circuits table with all rows
*/
fn circuits_csv(conn: &Connection, sql_rows: i64, filepath: String) {
    let file = File::open(filepath.clone()).expect("ERROR: Unable to access CSV");
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
    let csv_rows: usize = reader.records().count();
    if sql_rows != csv_rows as i64 {
        let file = File::open(filepath).expect("ERROR: Unable to access CSV");
        let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
        for row in reader.records() {
            let record = row.expect("ERROR: Unable to access row");
            let circuit_id = &record[0];
            let circuit_ref = &record[1];
            let name = &record[2];
            let location = &record[3];
            let country = &record[4];
            let lat = &record[5];
            let lng = &record[6];
            let alt = &record[7];
            let url = &record[8];

            conn.execute(
                "INSERT INTO circuits (
                    circuitId,
                    circuitRef,
                    name,
                    location,
                    country,
                    lat,
                    lng,
                    alt,
                    url
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)", 
                params![
                    circuit_id,circuit_ref,name,location,country,lat,lng,alt,url,
                    ],
                ).expect("ERROR: Unable to Insert Circuit Row");
        }
        println!("circuits TABLE POPULATED VIA CSV");
    } else {
        println!("circuits TABLE ALREADY UP-TO-DATE");
    }

}

/*
Reads the status.csv and populates the status table with all rows
*/
fn status_csv(conn: &Connection, sql_rows: i64, filepath: String) {
    let file = File::open(filepath.clone()).expect("ERROR: Unable to access CSV");
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
    let csv_rows: usize = reader.records().count();
    if sql_rows != csv_rows as i64 {
        let file = File::open(filepath).expect("ERROR: Unable to access CSV");
        let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
        for row in reader.records() {
            let record = row.expect("ERROR: Unable to access row");
            let status_id = &record[0];
            let status = &record[1];

            conn.execute(
                "INSERT INTO status (
                    statusId,
                    status
                ) VALUES (?1, ?2)", 
                params![
                    status_id,status,
                    ],
                ).expect("ERROR: Unable to Insert status Row");
        }
        println!("status TABLE POPULATED VIA CSV");
    } else {
        println!("circstatusuits TABLE ALREADY UP-TO-DATE");
    }

}

/*
Query all races of a given year into Races Objects
@return: vector of Races
*/
pub fn get_races(year: String) -> Result<Vec<Races>, Box<dyn Error>> {
    let conn: Connection = connect_to_db()?;
    let mut races_statement = conn.prepare("SELECT * FROM races WHERE year = ?1")?;
    let mut rows = races_statement.query([year])?;
    let mut races: Vec<Races> = Vec::new();

    while let Some(row) = rows.next()? {
        let race_id: i64 = row.get(0)?;
        let year: i64 = row.get(1)?;
        let round: i64 = row.get(2)?;
        let circuit_id: i64 = row.get(3)?;
        let name: String = row.get(4)?;
        let date: String = set_to_null_if_n(row.get(5)?);
        let time: String = set_to_null_if_n(row.get(6)?);
        let url: String = row.get(7)?;
        let fp1_date: String = set_to_null_if_n(row.get(8)?);
        let fp1_time: String = set_to_null_if_n(row.get(9)?);
        let fp2_date: String = set_to_null_if_n(row.get(10)?);
        let fp2_time: String = set_to_null_if_n(row.get(11)?);
        let fp3_date: String = set_to_null_if_n(row.get(12)?);
        let fp3_time: String = set_to_null_if_n(row.get(13)?);
        let quai_date: String = set_to_null_if_n(row.get(14)?);
        let quali_time: String = set_to_null_if_n(row.get(15)?);
        let sprint_date: String = set_to_null_if_n(row.get(16)?);
        let sprint_time: String = set_to_null_if_n(row.get(17)?);
        let mut circuits_statement = conn.prepare("SELECT country FROM circuits WHERE circuitId = ?1")?;
        let country: String = circuits_statement.query_row([circuit_id], |row| row.get(0))?;
        let country_code: String = get_country_code_country(country.clone());

        let race: Races = Races {
            race_id: race_id,
            year: year,
            round: round,
            circuit_id: circuit_id,
            name: name,
            date: date,
            time: time,
            url: url,
            fp1_date: fp1_date,
            fp1_time: fp1_time,
            fp2_date: fp2_date,
            fp2_time: fp2_time,
            fp3_date: fp3_date,
            fp3_time: fp3_time,
            quai_date: quai_date,
            quali_time: quali_time,
            sprint_date: sprint_date,
            sprint_time: sprint_time,
            country: country,
            country_code: country_code
        };
        races.push(race)
    }
    Ok(races)
}

/*
Given the driver_id, query the table drivers for that drivers information.
@returns: Object of the driver
*/
pub fn get_driver(driver_id: String) -> Result<Driver, rusqlite::Error> {
    // Connect to f1_db and get the driver from the table drivers where driver_id
    let conn: Connection = connect_to_db().expect("ERROR: Unable to connect to database");
    let mut driver_statement = conn.prepare("SELECT * FROM drivers WHERE driver_id = ?1")?;
    let mut rows = driver_statement.query([driver_id])?;

    // Create a Driver object
    let mut x: Option<Driver> = None;
    while let Some(row) = rows.next()? {
        let driver_id: i64 = row.get(0)?;
        let driver_ref: String = row.get(1)?;
        let number: i64 = row.get(2)?;
        let code: String = row.get(3)?;
        let forename: String = row.get(4)?;
        let surename: String = row.get(5)?;
        let dob: String = row.get(6)?;
        let nationality: String = row.get(7)?;
        let url: String = row.get(8)?;
        let country_code: String = get_country_code_nationality(nationality.clone());

        x = Some(Driver {
            driver_id: driver_id,
            driver_ref: driver_ref,
            number: number,
            code: code,
            forename: forename,
            surename: surename,
            dob: dob,
            nationality: nationality,
            url: url,
            country_code: country_code
        });
    }
    x.ok_or_else(|| rusqlite::Error::QueryReturnedNoRows)
}

/*
Given the circuit_id, query the table circuits for that circuit information.
@returns: Object of the driver
*/
pub fn get_circuit(circuit_id: String) -> Result<Circuit, rusqlite::Error> {
    // Connect to f1_db and get the driver from the table drivers where driver_id
    let conn: Connection = connect_to_db().expect("ERROR: Unable to connect to database");
    let mut driver_statement = conn.prepare("SELECT * FROM circuits WHERE circuit_id = ?1")?;
    let mut rows = driver_statement.query([circuit_id])?;

    // Create a Circuit object
    let mut x: Option<Circuit> = None;
    while let Some(row) = rows.next()? {
        let circuit_id: i64 = row.get(0)?;
        let circuit_ref: String = row.get(1)?;
        let name: String = row.get(2)?;
        let location: String = row.get(3)?;
        let country: String = row.get(4)?;
        let lat: String = row.get(5)?;
        let lng: String = row.get(6)?;
        let alt: String = row.get(7)?;
        let url: String = row.get(8)?;

        x = Some(Circuit {
            circuit_id: circuit_id,
            circuit_ref: circuit_ref,
            name: name,
            location: location,
            country: country,
            lat: lat,
            lng: lng,
            alt: alt,
            url: url
        });
    }
    x.ok_or_else(|| rusqlite::Error::QueryReturnedNoRows)
}

/*
@return: The nearest Grand Prix event for the homepage
Information Needed:
*   All data in the races table + country and countryId for flag
*/

#[derive(Debug)]
#[derive(Serialize)]
pub struct NextEvent {
    race_id: i64,
    round: i64,
    circuit_id: i64,
    grand_prix_name: String,
    grand_prix_date: String,
    grand_prix_time: String,
    fp1_date: String,
    fp1_time: String,
    fp2_date: String,
    fp2_time: String,
    fp3_date: String,
    fp3_time: String,
    sprint_date: String,
    sprint_time: String,
    quai_date: String,
    quali_time: String,
    next_event_name: String,
    next_event_time: String,
    country: String,
    location: String,
    country_code: String,
}

pub fn home_page_next_event() -> Result<NextEvent, rusqlite::Error>{
    let conn: Connection = connect_to_db().expect("ERROR: Unable to connect to database");
    let mut stmt = conn.prepare(
        "SELECT 
        r.raceId, r.round, r.circuitId, 
        r.name, r.date, r.time, r.fp1_date, r.fp1_time, r.fp2_date, r.fp2_time, r.fp3_date, r.fp3_time, 
        r.sprint_date, r.sprint_time, r.quali_date, r.quali_time, 
        c.country, c.location
        FROM races as r 
        JOIN circuits as c ON r.circuitId = c.circuitId
        WHERE date = (
            SELECT date 
            FROM races 
            WHERE date >= DATE('now') 
            ORDER BY date ASC 
            LIMIT 1
        )")?;
    let mut rows = stmt.query(params![])?;

    let mut next_event: Option<NextEvent> = None;
    while let Some(row) = rows.next()? {
        let race_id: i64 = row.get(0)?;
        let round: i64 = row.get(1)?;
        let circuit_id: i64 = row.get(2)?;
        let grand_prix_name: String = row.get(3)?;
        let grand_prix_date: String = row.get(4)?;
        let grand_prix_time: String = row.get(5)?;
        let fp1_date: String = row.get(6)?;
        let fp1_time: String = row.get(7)?;
        let fp2_date: String = row.get(8)?;
        let fp2_time: String = row.get(9)?;
        let fp3_date: String = row.get(10)?;
        let fp3_time: String = row.get(11)?;
        let quai_date: String = row.get(12)?;
        let quali_time: String = row.get(13)?;
        let sprint_date: String = row.get(14)?;
        let sprint_time: String = row.get(15)?;
        let country: String = row.get(16)?;
        let location: String = row.get(17)?;
        let country_code: String = get_country_code_country(country.clone());
        let mut next_event_name: String = String::from("NONE");
        let mut next_event_time: String = String::from("NONE");

        /*
        Given all the sub event times, determine which event is either Live or is going to Happen Next
        Live is anytime that is within the timeframe of 1 hour ago or ahead
        If No event is live, get the nearest future event
         */
        let events: Vec<(String, String)> = vec![
            (String::from("Free Practice 1"), to_datetime(fp1_date.clone(), fp1_time.clone())),
            (String::from("Free Practice 2"), to_datetime(fp2_date.clone(), fp2_time.clone())),
            (String::from("Free Practice 3"), to_datetime(fp3_date.clone(), fp3_time.clone())),
            (String::from("Free Practice 4"), to_datetime(sprint_date.clone(), sprint_time.clone())),
            (String::from("Free Practice 5"), to_datetime(quai_date.clone(), quali_time.clone())),
            (String::from("Free Practice 6"), to_datetime(grand_prix_date.clone(), grand_prix_time.clone())),
        ];

        let current_datetime = chrono::Local::now().naive_local();
        for (label, datetime_str) in events{
            let datetime = parse_datetime(&datetime_str);
            // See if time is within 2 hours from now
            let one_hour_ago = current_datetime + Duration::hours(-1);
            let one_hour_ahead = current_datetime + Duration::hours(1);

            println!("Current Datetime: {:}", current_datetime);
            println!("{:} {:} {:} {:}",label, datetime, one_hour_ago, one_hour_ahead);

            if datetime >= one_hour_ago && datetime <= one_hour_ahead {
                next_event_name = label;
                next_event_time = datetime.time().to_string();
                break
            }

            // If not, find the nearest event and time
            if datetime >= current_datetime{
                next_event_name = label;
                next_event_time = datetime.time().to_string();
                break
            }
        }


        next_event = Some(NextEvent {
            race_id: race_id,
            round: round,
            circuit_id: circuit_id,
            grand_prix_name: grand_prix_name,
            grand_prix_date: grand_prix_date,
            grand_prix_time: grand_prix_time,
            fp1_date: fp1_date,
            fp1_time: fp1_time,
            fp2_date: fp2_date,
            fp2_time: fp2_time,
            fp3_date: fp3_date,
            fp3_time: fp3_time,
            sprint_date: sprint_date,
            sprint_time: sprint_time,
            quai_date: quai_date,
            quali_time: quali_time,
            next_event_name: next_event_name,
            next_event_time: next_event_time,
            country: country,
            location: location,
            country_code: country_code
        });
    }
    println!("{:?}",next_event);
    next_event.ok_or_else(|| rusqlite::Error::QueryReturnedNoRows)
}

/*
@return: A Ascending DriverStanding Vector for the homepage
Information Needed:
*   DriverId, Poitns, Position, Wins, Forename, Surename, Nationality
*/
pub fn home_page_driver_standings() -> Result<Vec<DriverStanding>, rusqlite::Error> {
    let conn: Connection = connect_to_db().expect("ERROR: Unable to connect to database");
    let mut stmt = conn.prepare(
        "SELECT ds.driverId, ds.points, ds.position, ds.wins, drivers.forename, drivers.surename, drivers.nationality
        FROM driverStandings as ds
        JOIN drivers ON ds.driverId = drivers.driverId
        WHERE ds.raceId = (
            SELECT MAX(raceId)
            FROM driverStandings
        )
        ORDER BY position ASC
        ")?;
    let mut rows = stmt.query(params![])?;

    // Vector of Driver Objects
    let mut driver_standings: Vec<DriverStanding> = Vec::new();
    while let Some(row) = rows.next()? {
        let driver_id: i64 = row.get(0)?;
        let points: i64 = row.get(1)?;
        let position: i64 = row.get(2)?;
        let wins: i64 =  row.get(3)?;
        let forename: String = row.get(4)?;
        let surename: String = row.get(5)?;
        let nationality: String =  row.get(6)?;
        let country_code: String = get_country_code_nationality(nationality.clone());

        let driver: DriverStanding = DriverStanding {
            driver_id: driver_id,
            points: points,
            position: position,
            wins: wins,
            forename: forename,
            surename: surename,
            nationality: nationality,
            country_code: country_code
        };
        driver_standings.push(driver);
    }
    Ok(driver_standings)
}

/*
@return: A Ascending ConstructorStanding Vector for the homepage
Information Needed:
*   ConstructorId, Poitns, Position, Wins, Name, Nationality
*/
pub fn home_page_constructor_standings() -> Result<Vec<ConstructorStanding>, rusqlite::Error> {
    let conn: Connection = connect_to_db().expect("ERROR: Unable to connect to database");
    let mut stmt = conn.prepare(
        "SELECT cs.constructorId, cs.points, cs.position, cs.wins, constructors.name, constructors.nationality
        FROM constructorStandings as cs
        JOIN constructors ON cs.constructorId = constructors.constructorId
        WHERE cs.raceId = (
            SELECT MAX(raceId)
            FROM constructorStandings
        )
        ORDER BY position ASC
        ")?;
    let mut rows = stmt.query(params![])?;

    // Vector of ConstructorStanding Objects
    let mut constructor_standings: Vec<ConstructorStanding> = Vec::new();
    while let Some(row) = rows.next()? {
        let constructor_id: i64 = row.get(0)?;
        let points: i64 = row.get(1)?;
        let position: i64 = row.get(2)?;
        let wins: i64 =  row.get(3)?;
        let name: String = row.get(4)?;
        let nationality: String =  row.get(5)?;
        let country_code: String = get_country_code_nationality(nationality.clone());

        let constructor: ConstructorStanding = ConstructorStanding {
            constructor_id: constructor_id,
            points: points,
            position: position,
            wins: wins,
            name: name,
            nationality: nationality,
            country_code: country_code
        };
        constructor_standings.push(constructor);
    }
    Ok(constructor_standings)
}