use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from mobile endpoint!")
}

// #[get("/user/{id}/{name}/index.html")]
// async fn user_index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     format!("Welcome {}! id:{}", name, id)
// }
