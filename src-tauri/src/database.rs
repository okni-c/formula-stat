/*
TODO:
*   [] Get Country of Citcuit
*   [] Format Country to Country Code
*   [] Get the winner of the circuit if there is one

*/

use rusqlite::{Connection, params};
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Serialize;

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
}

pub fn connect_to_db() -> Result<Connection, Box<dyn Error>> {
    let conn: Connection = Connection::open("f1.db").expect("ERROR: Unable to connect to f1.db");
    Ok(conn)
}

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
    println!("TABLE CREATED: drivers");

    conn.close();
    Ok(())
}

pub fn populate_tables_via_csv() -> Result<(), Box<dyn Error>>{
    let conn: Connection = connect_to_db()?;
    // Races
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM races", [], |row| row.get(0))?;
    races_csv(&conn, sql_rows, String::from("data/races.csv"));

    // Drivers
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM drivers", [], |row| row.get(0))?;
    drivers_csv(&conn, sql_rows, String::from("data/drivers.csv"));

    //Circuits
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM circuits", [], |row| row.get(0))?;
    circuits_csv(&conn, sql_rows, String::from("data/circuits.csv"));

    conn.close();

    Ok(())
}

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

pub fn get_races(year: String) -> Result<Vec<Races>, Box<dyn Error>> {
    let conn: Connection = connect_to_db()?;
    
    let mut statement = conn.prepare("SELECT * FROM races WHERE year = ?1")?;
    let mut rows = statement.query([year])?;
    let mut races: Vec<Races> = Vec::new();
    while let Some(row) = rows.next()? {
        let race_id: i64 = row.get(0)?;
        let year: i64 = row.get(1)?;
        let round: i64 = row.get(2)?;
        let circuit_id: i64 = row.get(3)?;
        let name: String = row.get(4)?;
        let date: String = row.get(5)?;
        let time: String = row.get(6)?;
        let url: String = row.get(7)?;
        let fp1_date: String = row.get(8)?;
        let fp1_time: String = row.get(9)?;
        let fp2_date: String = row.get(10)?;
        let fp2_time: String = row.get(11)?;
        let fp3_date: String = row.get(12)?;
        let fp3_time: String = row.get(13)?;
        let quai_date: String = row.get(14)?;
        let quali_time: String = row.get(15)?;
        let sprint_date: String = row.get(16)?;
        let sprint_time: String = row.get(17)?;
        let x: Races = Races {
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
            sprint_time: sprint_time
        };
        races.push(x)
    }
    Ok(races)
}