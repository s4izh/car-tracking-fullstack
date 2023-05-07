// use crate::schema::*;
use diesel::{Queryable, Insertable};
use diesel::Expression;
use chrono::NaiveDateTime;
use crate::db::schema::{users,trips,car_data};
// use diesel::sql_types::Double;
use diesel::sql_types::Double;

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
    pub avg_speed: Double,
    pub max_speed: Double,
    pub fuel_percentage: Double,
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
    pub avg_speed: Double,
    pub max_speed: Double,
    pub fuel_percentage: Double,
    pub duration: i32,
    pub trouble_codes: &'a str,
}

#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = car_data)]
pub struct DbCarData{
    pub id: i32,
    pub timestamp: String,
    pub trouble_codes: String,
    pub speed: i32,
    pub rpm: i32,
    pub throttle: Double,
    pub engine_load: Double,
    pub engine_coolant_temp: Double,
    pub oil_temp: Double,
    pub fuel_level: Double,
    pub fuel_consumption: Double,
}

// #[derive(Insertable)]
// #[diesel(table_name = car_data)]
// pub struct NewDbCarData<'a> {
//     pub id: i32,
//     pub timestamp: &'a str,
//     pub speed: i32,
//     pub rpm: i32,
//     pub throttle: Double,
//     pub engine_load: Double,
//     pub engine_coolant_temp: Double,
//     pub oil_temp: Double,
//     pub fuel_level: Double,
//     pub fuel_consumption: Double,
// }
