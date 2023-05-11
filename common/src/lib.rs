use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct UserData {
    pub matricula: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct User {
    pub matricula: String,
    pub total_km: i32,
    pub trip: i32,
    pub date_created: chrono::NaiveDateTime,
    pub trip_data: Vec<Trip>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Trip {
    pub km: i32, // km
    pub max_speed: f64, // km/h
    pub speed_average: f64, // km/h
    pub fuel_percentage: f64, // fuel %
    pub duration: i32, // seconds
    pub trouble_codes: String,
    pub date_created: chrono::NaiveDateTime,
    pub data: Vec<Data>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Data {
    pub timestamp: String,
    pub speed: i32,
    pub rpm: i32,
    pub throttle: f64,
    pub engine_load: f64,
    pub engine_coolant_temp: f64,
    pub oil_temp: f64,
    pub fuel_level: f64,
    pub fuel_consumption: f64,
}
