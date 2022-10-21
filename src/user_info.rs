use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayInfo {
    pub date: DateTime<Utc>,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub key: String,
    pub account_created: Option<DateTime<Utc>>,
    pub from_server: Option<String>,
    pub play_info: Option<PlayInfo>,
}
