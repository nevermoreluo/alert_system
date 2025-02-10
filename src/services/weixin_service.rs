use reqwest::Client;
use crate::models::alert::Alert;
use super::alarm_service::Alarm;



pub struct WeChatAlarm {
    webhook: String
}



impl WeChatAlarm {
    pub fn new(webhook_url: &String) -> Self {
        WeChatAlarm {
            webhook: webhook_url.clone()
        }
    }
}

// #[async_trait::async_trait]
impl Alarm for WeChatAlarm {
    async fn send_alert(&self, alert: &Alert) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let message = serde_json::json!({
            "msg_type": "interactive",
            "card": alert.title.clone()
        });
        let client = Client::new();
    
        let response = client.post(&self.webhook)
            .json(&message)
            .send()
            .await?;
        Ok(response.json::<serde_json::Value>().await?)
    }
}
