use actix_web::{post, web, web::Json, HttpResponse, Responder};

use diesel::r2d2::{Pool, ConnectionManager};
use diesel::{MysqlConnection, RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::db::schema::users::dsl::users;
use crate::db::models;

use crate::db::models::BdUser;
use crate::db::schema::users::{matricula, hash};

#[post("/login")]
async fn login(
    user: Json<common::UserData>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>
) -> impl Responder {

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let result = users
        .filter(matricula.eq(&user.matricula))
        .filter(hash.eq(&user.hash))
        .first::<BdUser>(&mut *conn);
    match result {
        Err(diesel::NotFound) => return HttpResponse::BadRequest().body("User doesn't exists or bad password"),
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError()
            .body(format!("Error finding user")),
    }

    HttpResponse::Ok().body("Login successful")
}

#[post("/create-user")]
async fn create_user(
    user: Json<common::UserData>,
    pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let result = users
        .filter(matricula.eq(&user.matricula))
        .first::<BdUser>(&mut *conn);
    match result {
        Ok(_) => return HttpResponse::BadRequest().body("User already exists"),
        Err(diesel::NotFound) => (),
        Err(_) => return HttpResponse::InternalServerError()
            .body(format!("Error finding user")),
    }

    let new_user = models::NewBdUser {
        matricula: &user.matricula,
        hash: &user.hash,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut *conn)
        .map_err(|e| HttpResponse::InternalServerError()
        .body(format!("Error inserting user: {:?}", e)))
        .expect("Error inserting user");

    HttpResponse::Ok().body("User created")
}
