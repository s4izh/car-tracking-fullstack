use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CarData {
    pub matricula: String,
    pub hash: String,
    pub km: i32,
    pub max_speed: i32,
    pub speed_average: f64,
    pub duration: i32,
    pub data: Vec<CarGeneralData>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CarGeneralData {
    pub timestamp: String,
    pub trouble_codes: String,
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
