use actix_web::{test, App};
use my_alert_system::{routes::api_routes, utils::config::Config};

#[actix_rt::test]
async fn test_create_alert() {
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(Config {
                feishu_webhook: "mock_feishu_webhook".to_string(),
                wechat_webhook: "mock_wechat_webhook".to_string(),
            }))
            .configure(api_routes),
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/alerts")
        .set_json(&serde_json::json!({
            "title": "Test Alert",
            "message": "This is a test alert message."
        }))
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
}
