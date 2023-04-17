use actix_web::{get, web::Json, HttpResponse, Responder};

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
        matricula: "9999".to_string(),
        timestamp: "20230306".to_string(),
        trouble_codes: "".to_string(),
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
