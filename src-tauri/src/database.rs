/*
TODO:
*   [x] Get Country of Citcuit
*   [x] Get Country Code of Country

*/

use rusqlite::{Connection, params};
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use serde::Serialize;
use std::collections::HashMap;

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
    driver_id: String,
    driver_ref: String,
    number: i64,
    code: String,
    forename: String,
    surename: String,
    dob: String,
    nationality: String,
    url: String,
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
Going to create all tables if they dont exists
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

    // Drivers
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM drivers", [], |row| row.get(0))?;
    drivers_csv(&conn, sql_rows, String::from("data/drivers.csv"));

    //Circuits
    let sql_rows: i64 = conn.query_row("SELECT COUNT(*) FROM circuits", [], |row| row.get(0))?;
    circuits_csv(&conn, sql_rows, String::from("data/circuits.csv"));

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
Query all races of a given year into Races Objects
@return: vector of Races
*/
pub fn get_races(year: String) -> Result<Vec<Races>, Box<dyn Error>> {
    // Connect to f1_db and get all races in the given year
    let conn: Connection = connect_to_db()?;
    let mut races_statement = conn.prepare("SELECT * FROM races WHERE year = ?1")?;
    let mut rows = races_statement.query([year])?;
    let mut races: Vec<Races> = Vec::new();

    // Create a Races Object from each queried row and push it into a vector
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
        let mut circuits_statement = conn.prepare("SELECT country FROM circuits WHERE circuitId = ?1")?;
        let country: String = circuits_statement.query_row([circuit_id], |row| row.get(0))?;
        let country_code: String = get_country_code(country.clone());

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
            sprint_time: sprint_time,
            country: country,
            country_code: country_code
        };
        races.push(x)
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
        let driver_id: String = row.get(0)?;
        let driver_ref: String = row.get(1)?;
        let number: i64 = row.get(2)?;
        let code: String = row.get(3)?;
        let forename: String = row.get(4)?;
        let surename: String = row.get(5)?;
        let dob: String = row.get(6)?;
        let nationality: String = row.get(7)?;
        let url: String = row.get(8)?;

        x = Some(Driver {
            driver_id: driver_id,
            driver_ref: driver_ref,
            number: number,
            code: code,
            forename: forename,
            surename: surename,
            dob: dob,
            nationality: nationality,
            url: url
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

fn get_country_code(country:String) -> String {
    let mut country_dict: HashMap<&str, &str> = HashMap::new();
    country_dict.insert("Australia", "036");
    country_dict.insert("Malaysia", "458");
    country_dict.insert("Bahrain", "048");
    country_dict.insert("Spain", "724");
    country_dict.insert("Turkey", "792");
    country_dict.insert("Monaco", "492");
    country_dict.insert("Canada", "124");
    country_dict.insert("France", "250");
    country_dict.insert("UK", "826");
    country_dict.insert("Germany", "276");
    country_dict.insert("Hungary", "348");
    country_dict.insert("Belgium", "056");
    country_dict.insert("Italy", "380");
    country_dict.insert("Singapore", "702");
    country_dict.insert("Japan", "392");
    country_dict.insert("China", "156");
    country_dict.insert("Brazil", "076");
    country_dict.insert("USA", "840");
    country_dict.insert("United States", "840");
    country_dict.insert("UAE", "784");
    country_dict.insert("Argentina", "032");
    country_dict.insert("Portugal", "620");
    country_dict.insert("South Africa", "710");
    country_dict.insert("Mexico", "484");
    country_dict.insert("Korea", "410");
    country_dict.insert("Netherlands", "528");
    country_dict.insert("Sweden", "752");
    country_dict.insert("Austria", "040");
    country_dict.insert("Morocco", "504");
    country_dict.insert("Switzerland", "756");
    country_dict.insert("India", "356");
    country_dict.insert("Saudi Arabia", "682");
    country_dict.insert("Russia", "643");
    country_dict.insert("Azerbaijan", "031");
    country_dict.insert("Qatar", "634");
    country_dict.insert("Finland", "246");
    country_dict.insert("Poland", "616");
    country_dict.insert("Colombia", "170");
    country_dict.insert("Czechia", "203");
    country_dict.insert("Monaco", "492");
    country_dict.insert("New Zealand", "554");
    country_dict.insert("Denmakr", "208");
    country_dict.insert("Zimbabwe", "716");
    country_dict.insert("Venezuela", "862");
    country_dict.insert("Uruguay", "858");

    let code: String = country_dict.get(country.as_str()).unwrap_or(&"ERROR: Code not found").to_string();

    return code;
    
}