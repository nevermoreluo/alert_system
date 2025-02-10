use actix_web::web;
use crate::handlers::alert_handler;


/// Routes for the API
///
/// # Routes
///
/// - `POST /api/alerts`: send an alert to all the configured notification channels
pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/alerts").route(web::post().to(alert_handler::create_alert))),
    );
}
