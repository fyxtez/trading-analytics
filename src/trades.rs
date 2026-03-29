use chrono::Utc;
use std::collections::HashMap;

use crate::{parsing::parse_trade_event, types::{CleanMessage, Outcome, SignalKey, TradeEvent, TradeRecord}, utils::is_banned_symbol};

pub fn build_trade_records(clean_messages: &[CleanMessage]) -> Vec<TradeRecord> {
    let mut trades: HashMap<SignalKey, TradeRecord> = HashMap::new();

    for msg in clean_messages {
        let Some(event) = parse_trade_event(&msg.text) else {
            continue;
        };

        match event {
            TradeEvent::NewSignal {
                symbol,
                timeframe,
                entry,
            } => {
                if is_banned_symbol(&symbol) {
                    continue;
                }
                let key: SignalKey = (symbol.clone(), timeframe.clone(), entry.clone());

                trades.entry(key).or_insert_with(|| TradeRecord {
                    symbol,
                    timeframe,
                    entry,
                    opened_at: msg.date,
                    tp1_at: None,
                    tp2_at: None,
                    tp3_at: None,
                    sl_at: None,
                    first_outcome: None,
                    is_closed: false,
                });
            }

            TradeEvent::TakeProfit {
                symbol,
                timeframe,
                entry,
                level,
            } => {
                if is_banned_symbol(&symbol) {
                    continue;
                }
                let key: SignalKey = (symbol.clone(), timeframe.clone(), entry.clone());

                let Some(tr) = trades.get_mut(&key) else {
                    continue;
                };

                match level {
                    1 => {
                        tr.tp1_at.get_or_insert(msg.date.unwrap_or_else(Utc::now));
                        if tr.first_outcome.is_none() {
                            tr.first_outcome = Some(Outcome::TP1);
                        }
                    }
                    2 => {
                        tr.tp1_at.get_or_insert(msg.date.unwrap_or_else(Utc::now));
                        tr.tp2_at.get_or_insert(msg.date.unwrap_or_else(Utc::now));
                        if tr.first_outcome.is_none() {
                            tr.first_outcome = Some(Outcome::TP2);
                        }
                    }
                    3 => {
                        tr.tp1_at.get_or_insert(msg.date.unwrap_or_else(Utc::now));
                        tr.tp2_at.get_or_insert(msg.date.unwrap_or_else(Utc::now));
                        tr.tp3_at.get_or_insert(msg.date.unwrap_or_else(Utc::now));
                        if tr.first_outcome.is_none() {
                            tr.first_outcome = Some(Outcome::TP3);
                        }
                    }
                    _ => {}
                }
            }

            TradeEvent::StopLoss {
                symbol,
                timeframe,
                entry,
            } => {
                if is_banned_symbol(&symbol) {
                    continue;
                }
                let key: SignalKey = (symbol.clone(), timeframe.clone(), entry.clone());

                let Some(tr) = trades.get_mut(&key) else {
                    continue;
                };

                tr.sl_at.get_or_insert(msg.date.unwrap_or_else(Utc::now));
                if tr.first_outcome.is_none() {
                    tr.first_outcome = Some(Outcome::SL);
                }
                tr.is_closed = true;
            }
        }
    }

    let mut out: Vec<TradeRecord> = trades.into_values().collect();
    out.sort_by(|a, b| {
        (a.opened_at, &a.symbol, &a.timeframe, &a.entry).cmp(&(
            b.opened_at,
            &b.symbol,
            &b.timeframe,
            &b.entry,
        ))
    });
    out
}
