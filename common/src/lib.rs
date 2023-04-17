use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CarGeneralData {
    pub matricula: String,
    pub timestamp: String,
    pub trouble_codes: Vec<String>,
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
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub matricula: i32,
    pub main_user: i32,
}
