use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub feishu_webhook: String,
    pub wechat_webhook: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string("config.json")?;
        let config:Config  = serde_json::from_str(&contents)?;
        Ok(config)
        // config::Config::builder()
        //     .add_source(config::Environment::default())
        //     .build()?
        //     .try_deserialize()
    }
}
