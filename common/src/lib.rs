use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct TripData {
    pub matricula: String,
    pub hash: String,
    pub trip: i32, // numero
    pub km: i32, // km
    pub max_speed: i32, // km/h
    pub speed_average: f64, // km/h
    pub fuel_percentage: f64, // fuel %
    pub duration: i32, // seconds
    pub trouble_codes: String,
    pub data: Vec<CarData>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CarData {
    pub timestamp: String,
    pub speed: i32,
    pub rpm: i32,
    pub throttle: f32,
    pub engine_load: f32,
    pub engine_coolant_temp: f32,
    pub oil_temp: f32,
    pub fuel_level: f32,
    pub fuel_consumption: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct UserData {
    pub matricula: String,
    pub hash: String,
}
