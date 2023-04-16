use actix_web::{get, web::Json, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize, Debug)]
struct CarData {
    matricula: i32,
    speed: i32,
    rpm: i32,
    coolant_temp: i32,
    oil_temp: i32,
    throttle_pos: i32,
    engine_load: i32,
    fuel_level: i32,
    fuel_consumption: i32,
}

#[get("/test")]
async fn test() -> impl Responder {
    let car_data = Json(CarData {
        matricula: 9999,
        speed: 180,
        rpm: 3000,
        coolant_temp: 62,
        oil_temp: 90,
        throttle_pos: 15,
        engine_load: 80,
        fuel_level: 12,
        fuel_consumption: 8,
    });
    HttpResponse::Ok().json(car_data.into_inner())
}
