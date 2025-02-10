
use crate::models::alert::Alert;

pub trait Alarm {
    async fn send_alert(&self, alert: &Alert) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
}




