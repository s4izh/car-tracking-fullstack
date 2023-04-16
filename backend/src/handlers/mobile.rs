use actix_web::{get, post, web::Json, HttpResponse, Responder};

// use serde::{Deserialize, Serialize};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from mobile endpoint!")
}

#[post("/send-data")]
async fn send_data(car_data: Json<common::CarGeneralData>) -> impl Responder {
    HttpResponse::Ok().json(car_data.into_inner())
}

// #[get("/user/{id}/{name}/index.html")]
// async fn user_index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     format!("Welcome {}! id:{}", name, id)
// }
