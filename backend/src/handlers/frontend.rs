use actix_web::{get, post, web, web::Json, HttpResponse, Responder, error};

use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use diesel::RunQueryDsl;

use diesel::QueryDsl;
// use diesel::query_dsl::methods::FilterDsl;
// use std::iter::Iterator;


use crate::db::schema::users::dsl::users;

// mod db;
use crate::db::models;
use crate::db::schema;

// type DbPool = Pool<ConnectionManager<MysqlConnection>>;

// use serde::{Deserialize, Serialize};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from frontend!")
    // .content_type("text/html; charset=utf-8")
    // .body(include_str!("../static/index.html"))
}

#[get("/get-data")]
async fn get_data() -> impl Responder {
    HttpResponse::Ok()
}


#[get("/test")]
async fn test() -> impl Responder {
    let car_data = Json(common::CarGeneralData {
        // matricula: "9999".to_string(),
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
    HttpResponse::Ok().json(car_data.into_inner())
}

#[get("/certificate")]
// async fn certificate(pool: web::Data<DbPool>) -> Result<HttpResponse, actix_web::Error> {
// async fn certificate() -> Result<HttpResponse, actix_web::Error> {
async fn certificate(
    // user: web::Json<common::UserData>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
) -> Result<HttpResponse, actix_web::Error> {

    let conn = pool.get().expect("couldn't get db connection from pool");

    // verificar usuario

    // ------

    // let user = users
    //     .filter(matricula.eq(&data.matricula))
    //     .first::<User>(&conn)
    //     .optional()
    //     .expect("Error querying for user");

    // if let Some(user) = user {
    //     if user.hash == data.hash {
    //         Ok(HttpResponse::Ok().json(json!({"message": "User exists and hash matches"})))
    //     } else {
    //         Ok(HttpResponse::BadRequest().json(json!({"message": "Hash does not match"})))
    //     }
    // } else {
    //     Ok(HttpResponse::NotFound().json(json!({"message": "User not found"})))
    // }

    // conseguir kilometraje

    // getkm

    // ------
    let blockchain_client_url = std::env::var("BLOCKCHAIN_CLIENT_URL").expect("BLOCKCHAIN_CLIENT_URL must be set");

    let matricula = 0;
    let km = 1000;

    let url = format!("{}/certificate?matricula={}&km={}", 
                      blockchain_client_url, matricula, km);

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

// #[get("/login")]

use chrono::Utc;

// #[get("/create-user")]
#[post("/create-user")]
async fn create_user(
    user: Json<common::UserData>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>
) -> impl Responder {

    let user = common::UserData {
        matricula: "9999".to_string(),
        hash: "1234".to_string(),
    };

    let new_user = models::NewBdUser {
        matricula: &user.matricula,
        hash: &user.hash,
    };

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    diesel::insert_into(users)
        .values(&new_user)
        // .get_result(&mut *conn)
        .execute(&mut *conn)
        .map_err(|e| HttpResponse::InternalServerError()
                 .body(format!("Error inserting user: {:?}", e)));

    HttpResponse::Ok().json(user)
}
