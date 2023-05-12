use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub user: User,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct UserData2 {
    pub matricula: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct UserTrip {
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
    pub data: Vec<CarGeneralData>,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CarGeneralData {
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


#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub status: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

