use actix_web::{
    // web::Data,
    middleware::Logger,
    web::scope,
    App,
    HttpServer,
};
// use std::env::set_var;

mod api;

use api::front::get_data;
use api::mob::get_task;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // this is for logging into the terminal
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(
            scope("/api")
                .service(scope("/front").service(get_task))
                .service(scope("/mob").service(get_task)),
        )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
