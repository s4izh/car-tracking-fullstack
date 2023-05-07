use actix_web::{error, get, post, web, web::Json, HttpResponse, Responder};

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl};

// use crate::db::schema::users::dsl::users;
// use crate::db::schema::trips::dsl::trips;
// use crate::db::schema::car_data::dsl::car_data;
use crate::db::{models, schema};

// use crate::db::models;
// use crate::db::models::BdTrip;
// use crate::db::models::BdCarData;

use crate::db::schema::users::{hash, matricula, total_km};
// use crate::db::schema::trips;
// use crate::db::schema::car_data;
// use crate::db::schema::car_data::{id};

use diesel::prelude::*;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from frontend!")
}

#[get("/get-data")]
async fn get_data() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/test")]
async fn test() -> impl Responder {
    let data = Json(common::Data {
        timestamp: "20230306".to_string(),
        speed: 180,
        rpm: 3000,
        throttle: 15.0,
        engine_load: 80.0,
        engine_coolant_temp: 62.0,
        oil_temp: 90.0,
        fuel_level: 12.0,
        fuel_consumption: 8.0,
    });
    HttpResponse::Ok().json(data.into_inner())
}

#[post("/certificate")]
async fn certificate(
    user: web::Json<common::UserData>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // verificar usuario

    let result = schema::users::dsl::users
        .filter(matricula.eq(&user.matricula))
        .filter(hash.eq(&user.hash))
        .first::<models::BdUser>(&mut *conn);
    match result {
        Err(diesel::NotFound) => {
            return Ok(HttpResponse::BadRequest().body("User doesn't exists or bad password"))
        }
        Ok(_) => (),
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().body(format!("Error finding user")))
        }
    }

    // ------

    // get km

    let kilometers = result.as_ref().unwrap().total_km;
    let mat = &user.matricula;

    // ------

    // certificar

    let blockchain_client_url =
        std::env::var("BLOCKCHAIN_CLIENT_URL").expect("BLOCKCHAIN_CLIENT_URL must be set");

    let url = format!(
        "{}/certificate?matricula={}&km={}",
        blockchain_client_url, mat, kilometers
    );

    let response = reqwest::get(&url).await.map_err(|e| {
        // convert reqwest::Error into actix_web::Error
        error::ErrorInternalServerError(e)
    })?;

    let body = response.text().await.map_err(|e| {
        // convert reqwest::Error into actix_web::Error
        error::ErrorInternalServerError(e)
    })?;
    println!("{}", body);
    Ok(HttpResponse::Ok().body(body))
}

#[post("/get-trips")]
async fn get_trips(
    user: web::Json<common::UserData>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // verificar usuario

    let result = schema::users::dsl::users
        .filter(matricula.eq(&user.matricula))
        .filter(hash.eq(&user.hash))
        .first::<models::BdUser>(&mut *conn);
    match result {
        Err(diesel::NotFound) => {
            return Ok(HttpResponse::BadRequest().body("User doesn't exists or bad password"))
        }
        Ok(_) => (),
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().body(format!("Error finding user")))
        }
    }

    let mat = &user.matricula;
    let user_id = result.as_ref().unwrap().id;

    let trips_result = schema::trips::dsl::trips
        .filter(schema::trips::id.eq(user_id))
        .load::<models::BdTrip>(&mut *conn)
        .expect("Error loading trips");

    // For each trip, find all car data associated with it
    let trips = trips_result
        .into_iter()
        .map(|in_trip| {
            let car_data_result = schema::car_data::dsl::car_data
                .filter(schema::car_data::id.eq(user_id))
                .filter(schema::car_data::trip.eq(in_trip.trip))
                .load::<models::BdCarData>(&mut *conn)
                .expect("Error loading car data");

            let return_data = car_data_result
                .into_iter()
                .map(|bd_car_data| common::Data {
                    timestamp: bd_car_data.timestamp,
                    speed: bd_car_data.speed,
                    rpm: bd_car_data.rpm,
                    throttle: bd_car_data.throttle as f64 / 100.0,
                    engine_load: bd_car_data.engine_load as f64 / 100.0,
                    engine_coolant_temp: bd_car_data.engine_coolant_temp as f64 / 100.0,
                    oil_temp: bd_car_data.oil_temp as f64 / 100.0,
                    fuel_level: bd_car_data.fuel_level as f64 / 100.0,
                    fuel_consumption: bd_car_data.fuel_consumption as f64 / 100.0,
                })
                .collect::<Vec<common::Data>>();

            common::Trip {
                km: in_trip.km,
                max_speed: in_trip.max_speed,
                speed_average: in_trip.avg_speed as f64 / 100.0,
                fuel_percentage: in_trip.fuel_percentage as f64 / 100.0,
                duration: in_trip.duration,
                trouble_codes: in_trip.trouble_codes.clone(),
                date_created: in_trip.date_created,
                data: return_data,
            }
        })
        .collect::<Vec<common::Trip>>();

    let user_data = common::User {
        // matricula: mat.to_string();
        matricula: mat.to_string(),
        total_km: result.as_ref().unwrap().total_km,
        trip: result.as_ref().unwrap().trip,
        date_created: result.as_ref().unwrap().date_created,
        trip_data: trips,
    };

    // Ok(HttpResponse::Ok().json(user_data.into_inner()))
    Ok(HttpResponse::Ok().body("hola"))
}
