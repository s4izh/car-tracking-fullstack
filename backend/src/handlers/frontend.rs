use actix_web::{get, post, web, web::Json, HttpResponse, Responder, error};

use diesel::r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::ExpressionMethods;
use crate::db::schema::users::dsl::users;
use crate::db::models;
use crate::db::schema;

use crate::db::models::BdUser;
use crate::db::schema::users::{matricula, hash};

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
    let car_data = Json(common::CarData {
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

#[post("/certificate")]
async fn certificate(
    user: web::Json<common::UserData>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>,
) -> Result<HttpResponse, actix_web::Error> {

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // verificar usuario

    let result = users
        .filter(matricula.eq(&user.matricula))
        .filter(hash.eq(&user.hash))
        .first::<BdUser>(&mut *conn);
    match result {
        Err(diesel::NotFound) => return Ok(HttpResponse::BadRequest().body("User doesn't exists or bad password")),
        Ok(_) => (),
        Err(_) => return Ok(HttpResponse::InternalServerError()
            .body(format!("Error finding user"))),
    }

    // ------

    // get km

    // conseguir kilometraje

    // ------

    let blockchain_client_url = std::env::var("BLOCKCHAIN_CLIENT_URL").expect("BLOCKCHAIN_CLIENT_URL must be set");

    let mat = 0;
    let k = 1000;

    let url = format!("{}/certificate?matricula={}&km={}", 
                      blockchain_client_url, mat, k);

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
