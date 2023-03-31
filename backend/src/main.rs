use actix_web::{web, App, HttpServer};

mod routes;
use routes::frontend;
use routes::mobile;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // set DATABASE_URL and SERVER_PORT
    let _database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let backend_port: u16 = std::env::var("BACKEND_PORT")
        .expect("BACKEND_PORT must be set")
        .parse()
        .expect("BACKEND_PORT must be a number");

    HttpServer::new(move || {
        let logger = actix_web::middleware::Logger::default();
        App::new().wrap(logger).service(
            web::scope("/api")
                .service(web::scope("/frontend").service(frontend::index))
                .service(web::scope("/mobile").service(mobile::index)),
        )
    })
    .bind(("0.0.0.0", backend_port))?
    .run()
    .await
}
