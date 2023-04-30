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
// use serde::{Deserialize, Serialize};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from mobile endpoint!")
}

#[post("/send-data")]
async fn send_data(car_data: Json<common::CarData>) -> impl Responder {
    HttpResponse::Ok().json(car_data.into_inner())
}

#[post("/trip")]
async fn trip(
    trip_data: Json<common::TripData>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>
) -> impl Responder {




    HttpResponse::Ok().json(trip_data.into_inner())
}


// #[post("/create-user")]
// async fn create_user(
//     user: Json<common::UserData>,
//     pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>
// ) -> impl Responder {

//     // let user = common::UserData {
//     //     matricula: "9999".to_string(),
//     //     hash: "1234".to_string(),
//     // };
//     let mut conn = pool.get().expect("couldn't get db connection from pool");

//     let result = users
//         .filter(matricula.eq(&user.matricula))
//         .first::<BdUser>(&mut *conn);
//     match result {
//         Ok(_) => return HttpResponse::BadRequest().json("User already exists"),
//         Err(diesel::NotFound) => (),
//         Err(_) => return HttpResponse::InternalServerError()
//             .body(format!("Error finding user")),
//     }

//     let new_user = models::NewBdUser {
//         matricula: &user.matricula,
//         hash: &user.hash,
//     };

//     diesel::insert_into(users)
//         .values(&new_user)
//         // .get_result(&mut *conn)
//         .execute(&mut *conn)
//         .map_err(|e| HttpResponse::InternalServerError()
//                  .body(format!("Error inserting user: {:?}", e)));

//     HttpResponse::Ok().json(user)
// }


// #[post("/login")]
// async fn login(
//     user: Json<common::UserData>,
//     pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>
// ) -> impl Responder {

//     let mut conn = pool.get().expect("couldn't get db connection from pool");

//     let result = users
//         .filter(matricula.eq(&user.matricula))
//         .filter(hash.eq(&user.hash))
//         .first::<BdUser>(&mut *conn);
//     match result {
//         Err(diesel::NotFound) => return HttpResponse::BadRequest().body("User doesn't exists or bad password"),
//         Ok(_) => (),
//         Err(_) => return HttpResponse::InternalServerError()
//             .body(format!("Error finding user")),
//     }

//     let trip_num = 0;

//     HttpResponse::Ok().body(format!("{}",trip_num))
// }
