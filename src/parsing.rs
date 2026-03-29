use regex::Regex;
use serde_json::Value;

use crate::types::TradeEvent;

/// Flattens Telegram mixed text field into plain string.
pub fn normalize_text(text: Option<Value>) -> String {
    match text {
        Some(Value::String(s)) => s,

        Some(Value::Array(arr)) => arr
            .into_iter()
            .map(|v| {
                if let Some(s) = v.as_str() {
                    s.to_string()
                } else if let Some(obj_text) = v.get("text").and_then(|t| t.as_str()) {
                    obj_text.to_string()
                } else {
                    String::new()
                }
            })
            .collect(),

        _ => String::new(),
    }
}


pub fn parse_trade_event(text: &str) -> Option<TradeEvent> {
    // 🔔 New Signal
    if text.contains("🔔") {
        let re_header = Regex::new(r"🔔\s+([A-Z]+)\s+·\s+([0-9a-zA-Z]+)\s+·").unwrap();
        let re_entry = Regex::new(r"Entry:\s*([0-9.]+)").unwrap();

        let header = re_header.captures(text)?;
        let entry_cap = re_entry.captures(text)?;

        let symbol = header[1].to_string();
        let timeframe = header[2].to_string();
        let entry: f64 = entry_cap[1].parse().ok()?;

        return Some(TradeEvent::NewSignal {
            symbol,
            timeframe,
            entry: format!("{:.8}", entry),
        });
    }

    // 🎯 TP
    if text.contains("Current Trade") && text.contains("🎯 TP") {
        let re = Regex::new(
            r"(?s)🎯 TP([123])\s+([A-Z]+)\s+·.*?⏱️\s*([0-9a-zA-Z]+).*?Entry:\s*([0-9.]+)",
        )
        .unwrap();

        let cap = re.captures(text)?;

        let level: u8 = cap[1].parse().ok()?;
        let symbol = cap[2].to_string();
        let timeframe = cap[3].to_string();
        let entry: f64 = cap[4].parse().ok()?;

        return Some(TradeEvent::TakeProfit {
            symbol,
            timeframe,
            entry: format!("{:.8}", entry),
            level,
        });
    }

    // 🛡️ SL
    if text.contains("Current Trade") && text.contains("🛡️ SL") {
        let re = Regex::new(r"(?s)🛡️ SL\s+([A-Z]+)\s+·.*?⏱️\s*([0-9a-zA-Z]+).*?Entry:\s*([0-9.]+)")
            .unwrap();

        let cap = re.captures(text)?;

        let symbol = cap[1].to_string();
        let timeframe = cap[2].to_string();
        let entry: f64 = cap[3].parse().ok()?;

        return Some(TradeEvent::StopLoss {
            symbol,
            timeframe,
            entry: format!("{:.8}", entry),
        });
    }

    None
}