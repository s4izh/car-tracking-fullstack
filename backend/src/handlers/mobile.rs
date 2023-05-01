use actix_web::{get, post, web, web::Json, HttpResponse, Responder, error};

use diesel::{r2d2::{Pool, ConnectionManager}, MysqlConnection};
use diesel::{RunQueryDsl, QueryDsl};
use diesel::ExpressionMethods;
use crate::db::{models, schema};
use crate::db::schema::{users::dsl::users, trips::dsl::trips};

// use crate::db::schema::trips;
use crate::db::models::BdUser;
use crate::db::schema::users::{matricula, hash, trip, total_km};

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

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from mobile endpoint!")
}

#[post("/send-data")]
async fn send_data(car_data: Json<common::CarData>) -> impl Responder {
    HttpResponse::Ok().json(car_data.into_inner())
}

#[post("/trip")]
async fn add_trip(
    trip_data: Json<TripData>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>
) -> impl Responder {

    // check if user exists

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let result = users
        .filter(matricula.eq(&trip_data.matricula))
        .filter(hash.eq(&trip_data.hash))
        .first::<BdUser>(&mut *conn);
    match result {
        Err(diesel::NotFound) => return HttpResponse::BadRequest().body("User doesn't exists or bad password"),
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError()
            .body(format!("Error finding user")),
    }

    let user_id = result.as_ref().unwrap().id;
    let trip_num = result.as_ref().unwrap().trip;

    // add the new trip

    // let new_trip = models::NewBdTrip {
    //     id: user_id,
    //     trip: trip_num,
    //     km: trip_data.km,
    //     avg_speed: trip_data.speed_average,
    //     max_speed: trip_data.max_speed,
    //     fuel_percentage: trip_data.fuel_percentage,
    //     duration: trip_data.duration,
    //     trouble_codes: &trip_data.trouble_codes,
    // };
    // diesel::insert_into(trips)
    //     .values(&new_trip)
    //     .execute(&mut *conn)
    //     .expect("Error inserting trip");

    // increment the trip number

    let mat = &trip_data.matricula;

    diesel::update(users.filter(matricula.eq(mat)))
        .set(trip.eq(trip + 1))
        .execute(&mut *conn)
        .map_err(|e| HttpResponse::InternalServerError()
        .body(format!("Error updating user trip number: {:?}", e)))
        .expect("Error updating trip number");

    // increment the total km

    let km = &trip_data.km;

    diesel::update(users.filter(matricula.eq(mat)))
        .set(total_km.eq(total_km + km))
        .execute(&mut *conn)
        .map_err(|e| HttpResponse::InternalServerError()
        .body(format!("Error updating user total_km: {:?}", e)))
        .expect("Error updating user total_km");

    // add car_data

    // HttpResponse::Ok().json(trip_data.into_inner())
    HttpResponse::Ok().body(format!("{}", trip_num))
}
