// use crate::schema::*;
use diesel::{Queryable, Insertable};
use chrono::NaiveDateTime;
use crate::db::schema::users;
// use crate::db::schema::car_data;

#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = users)]
pub struct BdUser {
    pub id: i32,
    pub matricula: String,
    pub hash: String,
    pub total_km: i32,
    pub date_created: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewBdUser<'a> {
    pub matricula: &'a str,
    pub hash: &'a str,
}

// #[derive(Queryable, Debug, Clone)]
// #[table_name = "car_data"]
// pub struct DbCarData{
//     pub id: i32,
//     pub timestamp: String,
//     pub trouble_codes: String,
//     pub speed: i32,
//     pub rpm: i32,
//     pub throttle: f32,
//     pub engine_load: f32,
//     pub engine_coolant_temp: f32,
//     pub oil_temp: f32,
//     pub fuel_level: f32,
//     pub fuel_consumption: f32,
// }

// #[derive(Insertable)]
// #[table_name = "car_data"]
// pub struct NewDbCarData<'a> {
//     pub timestamp: &'a str,
//     pub trouble_codes: &'a str,
//     pub speed: i32,
//     pub rpm: i32,
//     pub throttle: f32,
//     pub engine_load: f32,
//     pub engine_coolant_temp: f32,
//     pub oil_temp: f32,
//     pub fuel_level: f32,
//     pub fuel_consumption: f32,
// }

