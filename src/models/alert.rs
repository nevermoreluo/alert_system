use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Alert {
    pub title: String,
    pub message: String,
    pub link: Option<String>
}
