use std::collections::HashMap;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

/*
Given a Country, returns the countrys code for Next.js
*/
pub fn get_country_code_country(country:String) -> String {
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
    country_dict.insert("New Zealand", "554");
    country_dict.insert("Denmark", "208");
    country_dict.insert("Zimbabwe", "716");
    country_dict.insert("Venezuela", "862");
    country_dict.insert("Uruguay", "858");

    let code: String = country_dict.get(country.as_str()).unwrap_or(&"ERROR: Code not found").to_string();

    return code;
    
}

/*
Given a Country, returns the countrys code for Next.js
*/
pub fn get_country_code_nationality(country:String) -> String {
    let mut country_dict: HashMap<&str, &str> = HashMap::new();
    country_dict.insert("Australian", "036");
    country_dict.insert("Malaysian", "458");
    country_dict.insert("Spanish", "724");
    country_dict.insert("Turkish", "792");
    country_dict.insert("Monegasque", "492");
    country_dict.insert("Canadian", "124");
    country_dict.insert("French", "250");
    country_dict.insert("British", "826");
    country_dict.insert("German", "276");
    country_dict.insert("Hungarian", "348");
    country_dict.insert("Belgian", "056");
    country_dict.insert("Italian", "380");
    country_dict.insert("Singaporean", "702");
    country_dict.insert("Japanese", "392");
    country_dict.insert("Chinese", "156");
    country_dict.insert("Brazilian", "076");
    country_dict.insert("American", "840");
    country_dict.insert("Argentinian", "032");
    country_dict.insert("Portuguese", "620");
    country_dict.insert("South African", "710");
    country_dict.insert("Mexican", "484");
    country_dict.insert("South Korean", "410");
    country_dict.insert("Dutch", "528");
    country_dict.insert("Swedish", "752");
    country_dict.insert("Austrian", "040");
    country_dict.insert("Moroccan", "504");
    country_dict.insert("Swiss", "756");
    country_dict.insert("Indian", "356");
    country_dict.insert("Saudi Arabian", "682");
    country_dict.insert("Russian", "643");
    country_dict.insert("Azerbaijani", "031");
    country_dict.insert("Qatari", "634");
    country_dict.insert("Finnish", "246");
    country_dict.insert("Polish", "616");
    country_dict.insert("Colombian", "170");
    country_dict.insert("Danish", "208");
    country_dict.insert("Zimbabwean", "716");
    country_dict.insert("Venezuelan", "862");
    country_dict.insert("Uruguayan", "858");

    let code: String = country_dict.get(country.as_str()).unwrap_or(&"ERROR: Code not found").to_string();

    return code;
    
}

pub fn set_to_null_if_n(temp: String) -> String {
    if temp == "\\N"{
        return String::from("None")
    } else {
        return temp
    }
}

pub fn to_datetime(date_str: String, time_str: String) -> String {
    if date_str == "\\N" {
        return String::from("None")
    }
    if let Ok(date) = parse_date(&date_str){
        if let Ok(time) = parse_time(&time_str) {
            let datetime: String = NaiveDateTime::new(date,time).to_string();
            return datetime
        } else {
            println!("ERROR: Unable to parse Time");
            return String::from("None")
        }
    } else {
        println!("ERROR: Unable to parse date");
        return String::from("None")
    }
}

fn parse_date(date_str: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
}

fn parse_time(time_str: &str) -> Result<NaiveTime, chrono::ParseError> {
    NaiveTime::parse_from_str(time_str, "%H:%M:%S")
}

pub fn parse_datetime(datetime_str: &str) -> NaiveDateTime {
    if datetime_str == "None" {
        return NaiveDateTime::parse_from_str("2000-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").expect("ERROR: Unable to Parse Datetime from String")
    }
    return NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S").expect("ERROR: Unable to Parse Datetime from String")
}

pub fn to_utc(time_str: &str) -> String {
    if time_str == "\\N"{
        return String::from("None")
    }
    let mut time: NaiveTime = NaiveTime::parse_from_str(time_str, "%H:%M:%S").unwrap();
    time = time + chrono::Duration::hours(1);
    return time.to_string()
}