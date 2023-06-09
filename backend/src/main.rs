use std::env;

use actix_web::web::scope;
use actix_web::{web, App, HttpServer};

use actix_cors::Cors;

use diesel::mysql::MysqlConnection;
// use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

mod handlers;
use handlers::frontend;
use handlers::mobile;
use handlers::common;

mod db;
// use db::utils;

pub async fn not_found(_req: actix_web::HttpRequest) -> impl actix_web::Responder {
    actix_web::HttpResponse::NotFound().body("404 Not Found")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // set DATABASE_URL and SERVER_PORT
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = {
        let manager: ConnectionManager<MysqlConnection> =
            ConnectionManager::<MysqlConnection>::new(&database_url);
        Pool::builder()
            .build(manager)
            .expect("Error building a connection pool")
    };
    let backend_port: u16 = env::var("BACKEND_PORT")
        .expect("BACKEND_PORT must be set")
        .parse()
        .expect("BACKEND_PORT must be a number");

    HttpServer::new(move || {
        // let _db_connection =
        //     MysqlConnection::establish(&database_url).expect("Error connecting to the database");
        let logger = actix_web::middleware::Logger::default();
        App::new()
            .wrap(logger)
            .wrap(Cors::permissive())
            .app_data(web::Data::new(db_pool.clone()))
            .service(
                scope("/api")
                    .service(
                        scope("/frontend")
                            .service(frontend::test)
                            .service(frontend::certificate)
                            .service(frontend::get_trips)
                    )
                    .service(
                        scope("/mobile")
                            .service(mobile::add_trip)
                    )
                    .service(common::create_user)
                    .service(common::login),
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
