pub mod parsing;
pub mod types;
pub mod trades;
pub mod utils;

use std::fs;

use crate::{
    parsing::normalize_text, trades::build_trade_records, types::{CleanMessage, TelegramExport}
};
use chrono::{NaiveDateTime, TimeZone, Utc};

// Move all this stuff to some bootstrap.rs or similar, so main does not need to be tested.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string("result.json")?;
    let export: TelegramExport = serde_json::from_str(&file_content)?;

    let mut clean_messages: Vec<CleanMessage> = Vec::new();

    for msg in export.messages {
        if msg.msg_type != "message" {
            continue;
        }
        if msg.from.as_deref() != Some("Liquidity Conceptives Signals") {
            continue;
        }

        let normalized = normalize_text(msg.text);
        if normalized.trim().is_empty() {
            continue;
        }

        let parsed_date = msg.date.as_deref().and_then(|d| {
            NaiveDateTime::parse_from_str(d, "%Y-%m-%dT%H:%M:%S")
                .ok()
                .map(|naive| Utc.from_utc_datetime(&naive))
        });

        clean_messages.push(CleanMessage {
            _id: msg.id,
            date: parsed_date,
            text: normalized,
        });
    }

    clean_messages.sort_by_key(|m| m.date);

    // let timeframe_order = ["30m", "1h"];
    // let timeframe_order = ["30m", "1h", "2h", "4h"];
    // let last_days: Option<i64> = None;


    let trades_all = build_trade_records(&clean_messages);

    dbg!(trades_all);

    Ok(())
}
