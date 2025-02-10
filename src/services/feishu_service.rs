use reqwest::Client;
use crate::models::alert::Alert;
use super::alarm_service::Alarm;
// use async_trait::async_trait;


pub struct FeiShuAlarm {
    webhook: String
}



impl FeiShuAlarm {
    pub fn new(webhook_url: &String) -> Self {
        FeiShuAlarm {
            webhook: webhook_url.clone()
        }
    }
}

// #[async_trait::async_trait]
impl Alarm for FeiShuAlarm {
    async fn send_alert(&self, alert: &Alert) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut elements = vec![serde_json::json!({
            "tag": "div",
            "text": {
                "content": alert.message,
                "tag": "lark_md"
            }
        })];
        // 如果 link 有值，则添加 actions 部分
        if let Some(ref link) = alert.link {
            elements.push(serde_json::json!({
                "actions": [{
                    "tag": "button",
                    "text": {
                        "content": "More",
                        "tag": "lark_md"
                    },
                    "url": link,
                    "type": "primary",
                    "value": {}
                }],
                "tag": "action"
            }));
        }

        let message = serde_json::json!({
            "msg_type": "interactive",
            "card": {
                "elements": elements,
                "header": {
                    "title": {
                        "content": alert.title,
                        "tag": "plain_text"
                    },
                    "template": "red" // 你可以根据需要更改模板颜色
                }
            }
        });
        let client = Client::new();
        let response = client.post(&self.webhook)
            .json(&message)
            .send()
            .await?;

        Ok(response.json::<serde_json::Value>().await?)
    }
}

