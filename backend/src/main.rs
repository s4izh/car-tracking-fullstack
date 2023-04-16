use std::env;

use actix_web::web::scope;
use actix_web::{web, App, HttpServer};

use handlebars::Handlebars;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

mod routes;
use routes::frontend;
use routes::mobile;

mod db;
// use db::utils;

pub async fn not_found(
    req: actix_web::HttpRequest,
    handlebars: web::Data<Handlebars<'_>>,
) -> impl actix_web::Responder {
    let data = serde_json::json!({ "url": &req.uri().to_string() });
    let body = handlebars.render("not-found", &data).unwrap();

    actix_web::HttpResponse::NotFound()
        // .status(actix_web::http::StatusCode::NOT_FOUND)
        .body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // set DATABASE_URL and SERVER_PORT

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let _db_connection =
        MysqlConnection::establish(&database_url).expect("Error connecting to the database");

    let backend_port: u16 = env::var("BACKEND_PORT")
        .expect("BACKEND_PORT must be set")
        .parse()
        .expect("BACKEND_PORT must be a number");

    // templating system for default responses
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./backend/templates/")
        .unwrap();

    let data = web::Data::new(handlebars);

    HttpServer::new(move || {
        let logger = actix_web::middleware::Logger::default();
        App::new()
            .wrap(logger)
            .app_data(data.clone())
            .service(
                scope("/api")
                    .service(
                        scope("/frontend")
                            .service(frontend::index)
                            .service(frontend::get_data)
                            .service(frontend::test),
                    )
                    .service(
                        scope("/mobile")
                            .service(mobile::index)
                            .service(mobile::send_data),
                    ),
            )
            .default_service(web::route().to(not_found))
        // .service(
        //     actix_web_lab::web::spa()
        //         .index_file("./../../frontend/index.html")
        //         .static_resources_mount("/")
        //         // .static_resources_location
        //         .finish(),
        // )
    })
    .bind(("0.0.0.0", backend_port))?
    .run()
    .await
}
