use actix_web::{
    // web::Data,
    middleware::Logger,
    web::scope,
    App,
    HttpServer,
};
use std::env::set_var;

mod api;

use api::front::get_data;
use api::mob::get_task;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // this is for logging into the terminal
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let database_url = std::env!("DATABASE_URL");
    let server_port = std::env!("BACKEND_PORT")
        .parse::<u16>()
        .expect("Port must be a u16");

    let app_state = web::Data::new(AppState {
        pool: sqlx::pool::PoolOptions::new()
            .connect(&database_url)
            .await
            .expect("Could not connect to the DB"),
    });

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(
            scope("/api")
                .service(scope("/front").service(get_data))
                .service(scope("/mob").service(send_data)),
        )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await

    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(app_state.clone())
    //         .wrap(actix_web::middleware::Logger::default())
    //         .wrap(actix_cors::Cors::permissive())
    //         .route("/", web::get().to(memo::index))
    //         .route("/", web::post().to(memo::create))
    //         .route("/", web::put().to(memo::resolve))
    //         .route("/", web::delete().to(memo::delete))
    // })
}
