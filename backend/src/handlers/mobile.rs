use actix_web::{get, post, web::Json, HttpResponse, Responder};

use diesel::r2d2::Pool;
// use serde::{Deserialize, Serialize};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from mobile endpoint!")
}

#[post("/send-data")]
async fn send_data(car_data: Json<common::CarGeneralData>) -> impl Responder {
// async fn send_data(car_data: Json<common::CarGeneralData>, db: web::Data<Pool>) -> impl Responder {
    // let conn = db.get().unwrap();
    // let new_user = NewUser {
    //     first_name: &item.first_name,
    //     last_name: &item.last_name,
    //     email: &item.email,
    //     created_at: chrono::Local::now().naive_local(),
    // };
    // let res = insert_into(users).values(&new_user).get_result(&conn)?;
    HttpResponse::Ok().json(car_data.into_inner())
}

// #[get("/user/{id}/{name}/index.html")]
// async fn user_index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     format!("Welcome {}! id:{}", name, id)
// }
