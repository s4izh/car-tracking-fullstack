use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from frontend!")
    // .content_type("text/html; charset=utf-8")
    // .body(include_str!("../static/index.html"))
}
