use actix_web::{web, HttpResponse, http::StatusCode};
use serde::Deserialize;
use crate::models::alert::Alert;
use crate::services::alarm_service::Alarm;
use crate::services::feishu_service::FeiShuAlarm;
use crate::services::weixin_service::WeChatAlarm;

#[derive(Deserialize)]
pub struct AlertRequest {
    title: String,
    message: String,
    link: Option<String>,
    #[serde(default)] // 如果 JSON 中没有提供该字段，则使用默认值 None
    alert_type: Option<AlertType>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum AlertType {
    FeiShu,
    WeChat,
    // DingTalk,
}

impl Default for AlertType {
    fn default() -> Self {
        AlertType::FeiShu
    }
}


pub async fn create_alert(
    data: web::Data<crate::utils::config::Config>,
    req: web::Json<AlertRequest>,
) -> HttpResponse {
    let alert = Alert {
        title: req.title.clone(),
        message: req.message.clone(),
        link: req.link.clone()
    };

    // 初始化一个空的 JSON 对象来存储报警器响应
    let alert_response: serde_json::Value;
    
    // 根据 alert_type 选择报警器
    match &req.alert_type.clone().unwrap_or_default() {
        AlertType::FeiShu => {
            let feishu = FeiShuAlarm::new(&data.feishu_webhook);

            match feishu.send_alert(&alert).await {
                Ok(response_data) => {
                    // 将 WeChat 服务器的响应数据合并到总的响应中
                    alert_response = response_data;
                },
                Err(e) => {
                    // 将详细的错误信息返回给客户端
                    return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                        .content_type("application/json")
                        .body(serde_json::json!({"error": format!("Failed to send to FeiShu: {}", e)}).to_string());
                }
            }
        }
        AlertType::WeChat => {
            let wechat = WeChatAlarm::new(&data.wechat_webhook);
            match wechat.send_alert(&alert).await {
                Ok(response_data) => {
                    // 将 WeChat 服务器的响应数据合并到总的响应中
                    alert_response = response_data;
                },
                Err(e) => {
                    // 将详细的错误信息返回给客户端
                    return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                        .content_type("application/json")
                        .body(serde_json::json!({"error": format!("Failed to send to WeChat: {}", e)}).to_string());
                }
            }
        }
        // AlertType::DingTalk => {
        //     // let dingtalk = DingTalkAlarm::new(&data.dingtalk_webhook);
        //     // if let Err(e) = dingtalk.send_alert(&alert).await {
        //     //     return HttpResponse::InternalServerError().body(format!("Failed to send to DingTalk: {}", e));
        //     // }
        // }
    }

    // 构造最终的响应 JSON，包括原始警报信息和报警器响应
    let final_response = serde_json::json!({
        "alert": alert,
        "alert_response": alert_response,
    });

    // 成功发送后返回 200 OK 和 JSON 格式的综合信息
    HttpResponse::Ok().json(final_response)
}
