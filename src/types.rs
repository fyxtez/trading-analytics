use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]

pub struct TelegramExport {
    pub messages: Vec<TelegramMessage>,
}

#[derive(Debug, Deserialize)]
pub struct TelegramMessage {
    pub id: i64,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub date: Option<String>,
    pub from: Option<String>,
    pub text: Option<Value>,
}

#[derive(Debug)]
pub struct CleanMessage {
    pub _id: i64,
    pub date: Option<DateTime<Utc>>,
    pub text: String,
}
