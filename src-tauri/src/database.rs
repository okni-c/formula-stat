use rusqlite::{Connection, params};
use tauri::AppHandle;
use std::{error::Error, vec};
use serde::Serialize;
use chrono::Duration;

use crate::func::{get_country_code_country, get_country_code_nationality, set_to_null_if_n, to_datetime, parse_datetime, to_utc};

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
pub fn connect_to_db(app_handle: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let sqlite_path = app_handle.path_resolver().resolve_resource("data/f1.db").expect("ERROR: Unable to access data/f1.db");
    println!("{:?}",sqlite_path);
    let conn = Connection::open(sqlite_path)?;

    println!("Connected to F1.db");
    Ok(conn)
}

/*
Query all races of a given year into Races Objects
@return: vector of Races
*/
pub fn get_races(year: String, conn: &Connection) -> Result<Vec<Races>, Box<dyn Error>> {
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
pub fn get_driver(driver_id: String, conn: &Connection) -> Result<Driver, rusqlite::Error> {
    // Connect to f1_db and get the driver from the table drivers where driver_id
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
pub fn get_circuit(circuit_id: String, conn: &Connection) -> Result<Circuit, rusqlite::Error> {
    // Connect to f1_db and get the driver from the table drivers where driver_id
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
    quali_date: String,
    quali_time: String,
    next_event_name: String,
    next_event_time: String,
    country: String,
    location: String,
    country_code: String,
}

pub fn home_page_next_event(conn: &Connection) -> Result<NextEvent, rusqlite::Error>{
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
        let mut grand_prix_time: String = row.get(5)?;
        grand_prix_time = to_utc(grand_prix_time.as_str());

        let fp1_date: String = row.get(6)?;
        let mut fp1_time: String = row.get(7)?;
        fp1_time = to_utc(fp1_time.as_str());

        let fp2_date: String = row.get(8)?;
        let mut fp2_time: String = row.get(9)?;
        fp2_time = to_utc(fp2_time.as_str());

        let fp3_date: String = row.get(10)?;
        let mut fp3_time: String = row.get(11)?;
        fp3_time = to_utc(fp3_time.as_str());

        let sprint_date: String = row.get(12)?;
        let mut sprint_time: String = row.get(13)?;
        sprint_time = to_utc(sprint_time.as_str());

        let quali_date: String = row.get(14)?;
        let mut quali_time: String = row.get(15)?;
        quali_time = to_utc(quali_time.as_str());

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
            (String::from("Sprint"), to_datetime(sprint_date.clone(), sprint_time.clone())),
            (String::from("Qualifying"), to_datetime(quali_date.clone(), quali_time.clone())),
            (grand_prix_name.clone(), to_datetime(grand_prix_date.clone(), grand_prix_time.clone())),
        ];

        let current_datetime = chrono::Local::now().naive_local();
        for (label, datetime_str) in events{
            let datetime = parse_datetime(&datetime_str);
            // See if time is within 2 hours from now
            let one_hour_ago = current_datetime + Duration::hours(-1);
            let one_hour_ahead = current_datetime + Duration::hours(1);

            if datetime >= one_hour_ago && datetime <= one_hour_ahead {
                next_event_name = label;
                next_event_time = datetime.to_string();
                break
            }

            // If not, find the nearest event and time
            if datetime >= current_datetime{
                next_event_name = label;
                next_event_time = datetime.to_string();
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
            quali_date: quali_date,
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
    driver_ref: String,
    country_code: String,
}
/*
@return: A Ascending DriverStanding Vector for the homepage
Information Needed:
*   DriverId, Poitns, Position, Wins, Forename, Surename, Nationality
*/
pub fn home_page_driver_standings(conn: &Connection) -> Result<Vec<DriverStanding>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT ds.driverId, ds.points, ds.position, ds.wins, drivers.forename, drivers.surename, drivers.nationality, drivers.driverRef
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
        let driver_ref: String =  row.get(7)?;
        let country_code: String = get_country_code_nationality(nationality.clone());

        let driver: DriverStanding = DriverStanding {
            driver_id: driver_id,
            points: points,
            position: position,
            wins: wins,
            forename: forename,
            surename: surename,
            nationality: nationality,
            driver_ref: driver_ref,
            country_code: country_code
        };
        driver_standings.push(driver);
    }
    println!("{:?}",driver_standings);
    Ok(driver_standings)
}

/*
@return: A Ascending ConstructorStanding Vector for the homepage
Information Needed:
*   ConstructorId, Poitns, Position, Wins, Name, Nationality
*/
pub fn home_page_constructor_standings(conn: &Connection) -> Result<Vec<ConstructorStanding>, rusqlite::Error> {
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
    println!("{:?}",constructor_standings);
    Ok(constructor_standings)
}