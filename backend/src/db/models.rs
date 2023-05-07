// use crate::schema::*;
use diesel::{Queryable, Insertable};
use diesel::Expression;
use chrono::NaiveDateTime;
use crate::db::schema::{users,trips,car_data};
// use diesel::sql_types::i32;
// use diesel::sql_types::i32;
// use core::i32;

#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = users)]
pub struct BdUser {
    pub id: i32,
    pub matricula: String,
    pub hash: String,
    pub total_km: i32,
    pub trip: i32,
    pub date_created: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewBdUser<'a> {
    pub matricula: &'a str,
    pub hash: &'a str,
}

#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = trips)]
pub struct BdTrip {
    pub id: i32,
    pub trip: i32,
    pub km: i32,
    pub avg_speed: i32,
    pub max_speed: i32,
    pub fuel_percentage: i32,
    pub duration: i32,
    pub trouble_codes: String,
    pub date_created: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = trips)]
pub struct NewBdTrip<'a> {
    pub id: i32,
    pub trip: i32,
    pub km: i32,
    pub avg_speed: i32,
    pub max_speed: i32,
    pub fuel_percentage: i32,
    pub duration: i32,
    pub trouble_codes: &'a str,
}

#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = car_data)]
pub struct BdCarData{
    pub id: i32,
    pub timestamp: String,
    pub trouble_codes: String,
    pub speed: i32,
    pub rpm: i32,
    pub throttle: i32,
    pub engine_load: i32,
    pub engine_coolant_temp: i32,
    pub oil_temp: i32,
    pub fuel_level: i32,
    pub fuel_consumption: i32,
}

#[derive(Insertable)]
#[diesel(table_name = car_data)]
pub struct NewBdCarData<'a> {
    pub id: i32,
    pub trip: i32,
    pub timestamp: &'a str,
    pub speed: i32,
    pub rpm: i32,
    pub throttle: i32,
    pub engine_load: i32,
    pub engine_coolant_temp: i32,
    pub oil_temp: i32,
    pub fuel_level: i32,
    pub fuel_consumption: i32,
}
