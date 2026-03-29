use serde_json::json;
use trading_analytics::parsing::{normalize_text, parse_trade_event};
use trading_analytics::types::TradeEvent;

#[test]
fn test_normalize_text_with_plain_string() {
    let input = Some(json!("Hello world"));
    let result = normalize_text(input);

    assert_eq!(result, "Hello world");
}

#[test]
fn test_normalize_text_with_array_of_strings() {
    let input = Some(json!(["Hello", " ", "world"]));
    let result = normalize_text(input);

    assert_eq!(result, "Hello world");
}

#[test]
fn test_normalize_text_with_array_of_objects_and_strings() {
    let input = Some(json!([
        {"type": "bold", "text": "BTCUSDT"},
        " ",
        {"type": "plain", "text": "LONG"}
    ]));
    let result = normalize_text(input);

    assert_eq!(result, "BTCUSDT LONG");
}

#[test]
fn test_normalize_text_with_unknown_array_items() {
    let input = Some(json!([
        {"foo": "bar"},
        123,
        true,
        {"text": "valid"}
    ]));
    let result = normalize_text(input);

    assert_eq!(result, "valid");
}

#[test]
fn test_normalize_text_with_none() {
    let result = normalize_text(None);
    assert_eq!(result, "");
}

#[test]
fn test_normalize_text_with_non_string_non_array() {
    let input = Some(json!(12345));
    let result = normalize_text(input);

    assert_eq!(result, "");
}

#[test]
fn test_parse_new_signal() {
    let input = "🔔 BTCUSDT · 1h · 🟢 LONG\nEntry: 50000";
    let result = parse_trade_event(input);

    match result {
        Some(TradeEvent::NewSignal {
            symbol,
            timeframe,
            entry,
        }) => {
            assert_eq!(symbol, "BTCUSDT");
            assert_eq!(timeframe, "1h");
            assert_eq!(entry, "50000.00000000");
        }
        _ => panic!("Expected NewSignal"),
    }
}

#[test]
fn test_parse_new_signal_with_decimal_entry() {
    let input = "🔔 ETHUSDT · 15m · 🟢 LONG\nEntry: 2450.75";
    let result = parse_trade_event(input);

    match result {
        Some(TradeEvent::NewSignal {
            symbol,
            timeframe,
            entry,
        }) => {
            assert_eq!(symbol, "ETHUSDT");
            assert_eq!(timeframe, "15m");
            assert_eq!(entry, "2450.75000000");
        }
        _ => panic!("Expected NewSignal"),
    }
}

#[test]
fn test_parse_tp() {
    let input = "Current Trade\n🎯 TP1 BTCUSDT · ⏱️ 1h Entry: 50000";
    let result = parse_trade_event(input);

    match result {
        Some(TradeEvent::TakeProfit {
            symbol,
            timeframe,
            level,
            entry,
        }) => {
            assert_eq!(symbol, "BTCUSDT");
            assert_eq!(timeframe, "1h");
            assert_eq!(level, 1);
            assert_eq!(entry, "50000.00000000");
        }
        _ => panic!("Expected TakeProfit"),
    }
}

#[test]
fn test_parse_tp2() {
    let input = "Current Trade\n🎯 TP2 ETHUSDT · something here ⏱️ 4h Entry: 2500.5";
    let result = parse_trade_event(input);

    match result {
        Some(TradeEvent::TakeProfit {
            symbol,
            timeframe,
            level,
            entry,
        }) => {
            assert_eq!(symbol, "ETHUSDT");
            assert_eq!(timeframe, "4h");
            assert_eq!(level, 2);
            assert_eq!(entry, "2500.50000000");
        }
        _ => panic!("Expected TakeProfit"),
    }
}

#[test]
fn test_parse_tp3() {
    let input = "Current Trade\n🎯 TP3 SOLUSDT · extra text ⏱️ 30m Entry: 150";
    let result = parse_trade_event(input);

    match result {
        Some(TradeEvent::TakeProfit {
            symbol,
            timeframe,
            level,
            entry,
        }) => {
            assert_eq!(symbol, "SOLUSDT");
            assert_eq!(timeframe, "30m");
            assert_eq!(level, 3);
            assert_eq!(entry, "150.00000000");
        }
        _ => panic!("Expected TakeProfit"),
    }
}

#[test]
fn test_parse_sl() {
    let input = "Current Trade\n🛡️ SL BTCUSDT · ⏱️ 1h Entry: 50000";
    let result = parse_trade_event(input);

    match result {
        Some(TradeEvent::StopLoss {
            symbol,
            timeframe,
            entry,
        }) => {
            assert_eq!(symbol, "BTCUSDT");
            assert_eq!(timeframe, "1h");
            assert_eq!(entry, "50000.00000000");
        }
        _ => panic!("Expected StopLoss"),
    }
}

#[test]
fn test_parse_sl_with_decimal_entry() {
    let input = "Current Trade\n🛡️ SL XRPUSDT · random stuff ⏱️ 5m Entry: 0.6245";
    let result = parse_trade_event(input);

    match result {
        Some(TradeEvent::StopLoss {
            symbol,
            timeframe,
            entry,
        }) => {
            assert_eq!(symbol, "XRPUSDT");
            assert_eq!(timeframe, "5m");
            assert_eq!(entry, "0.62450000");
        }
        _ => panic!("Expected StopLoss"),
    }
}

#[test]
fn test_parse_invalid() {
    let input = "random garbage text";
    let result = parse_trade_event(input);
    assert!(result.is_none());
}

#[test]
fn test_parse_new_signal_missing_entry_returns_none() {
    let input = "🔔 BTCUSDT · 1h · 🟢 LONG";
    let result = parse_trade_event(input);

    assert!(result.is_none());
}

#[test]
fn test_parse_tp_missing_current_trade_returns_none() {
    let input = "🎯 TP1 BTCUSDT · ⏱️ 1h Entry: 50000";
    let result = parse_trade_event(input);

    assert!(result.is_none());
}

#[test]
fn test_parse_sl_missing_current_trade_returns_none() {
    let input = "🛡️ SL BTCUSDT · ⏱️ 1h Entry: 50000";
    let result = parse_trade_event(input);

    assert!(result.is_none());
}

#[test]
fn test_parse_new_signal_invalid_entry_returns_none() {
    let input = "🔔 BTCUSDT · 1h · 🟢 LONG\nEntry: abc";
    let result = parse_trade_event(input);

    assert!(result.is_none());
}
