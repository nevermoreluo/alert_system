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
    // 提前提取 host_ip 和 port
    let host_ip: String = config.host_ip.clone();
    let port: String = config.port.clone();
    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .configure(routes::api_routes::api_routes)
    })
    .bind(format!("{}:{}", host_ip, port))?
    .run()
    .await
}

