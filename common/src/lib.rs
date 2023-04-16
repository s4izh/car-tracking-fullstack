use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CarGeneralData {
    pub matricula: i32,
    pub timestamp: i32,
    pub speed: i32,
    pub rpm: i32,
    pub coolant_temp: i32,
    pub oil_temp: i32,
    pub throttle_pos: i32,
    pub engine_load: i32,
    pub fuel_level: i32,
    pub fuel_consumption: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserData {
    pub matricula: i32,
    pub main_user: i32,
}
