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

#[derive(Debug, Clone)]
pub struct TradeRecord {
    pub symbol: String,
    pub timeframe: String,
    pub entry: String,

    pub opened_at: Option<DateTime<Utc>>,
    pub tp1_at: Option<DateTime<Utc>>,
    pub tp2_at: Option<DateTime<Utc>>,
    pub tp3_at: Option<DateTime<Utc>>,
    pub sl_at: Option<DateTime<Utc>>,

    pub first_outcome: Option<Outcome>,
    pub is_closed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Outcome {
    TP1,
    TP2,
    TP3,
    SL,
}

#[derive(Debug)]
pub enum TradeEvent {
    NewSignal {
        symbol: String,
        timeframe: String,
        entry: String,
    },
    TakeProfit {
        symbol: String,
        timeframe: String,
        entry: String,
        level: u8,
    },
    StopLoss {
        symbol: String,
        timeframe: String,
        entry: String,
    },
}

pub type SignalKey = (String, String, String); // (symbol, timeframe, entry)
