use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod handlers;
mod models;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = utils::config::Config::from_env().unwrap();

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .configure(routes::api_routes::api_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

