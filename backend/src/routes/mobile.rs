use actix_web::{get, post, web::Json, HttpResponse, Responder};

use serde::{Serialize,Deserialize};

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

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from mobile endpoint!")
}

#[post("/send-data")]
async fn send_data(car_data: Json<CarData>) -> impl Responder {
    HttpResponse::Ok()
        .json(car_data.into_inner())
}

// #[get("/user/{id}/{name}/index.html")]
// async fn user_index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     format!("Welcome {}! id:{}", name, id)
// }
