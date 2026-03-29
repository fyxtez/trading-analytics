use trading_analytics::{
    parsing::load_clean_messages_from_file,
    trades::build_trade_records,
};

#[test]
fn builds_trade_records_from_real_export() {
    let clean_messages = load_clean_messages_from_file("result.json").unwrap();
    let records = build_trade_records(&clean_messages);

    assert!(
        !clean_messages.is_empty(),
        "clean_messages should not be empty"
    );
    assert!(!records.is_empty(), "trade records should not be empty");
}

#[test]
fn all_trade_keys_are_non_empty() {
    let clean_messages = load_clean_messages_from_file("result.json").unwrap();
    let records = build_trade_records(&clean_messages);

    assert!(!records.is_empty(), "trade records should not be empty");

    for record in records {
        assert!(
            !record.symbol.trim().is_empty(),
            "symbol should not be empty"
        );
        assert!(
            !record.timeframe.trim().is_empty(),
            "timeframe should not be empty"
        );
        assert!(
            !record.entry.trim().is_empty(),
            "entry should not be empty"
        );
    }
}

#[test]
fn no_banned_symbols_exist_in_results() {
    let clean_messages = load_clean_messages_from_file("result.json").unwrap();
    let records = build_trade_records(&clean_messages);

    for record in records {
        assert_ne!(record.symbol, "ASTERUSDT");
        assert_ne!(record.symbol, "ASTER");
    }
}

#[test]
fn closed_trades_have_stop_loss_timestamp() {
    let clean_messages = load_clean_messages_from_file("result.json").unwrap();
    let records = build_trade_records(&clean_messages);

    for record in records {
        if record.is_closed {
            assert!(
                record.sl_at.is_some(),
                "closed trade should have sl_at set for {} {} {}",
                record.symbol,
                record.timeframe,
                record.entry
            );
        }
    }
}

#[test]
fn first_outcome_is_consistent_with_record_state() {
    let clean_messages = load_clean_messages_from_file("result.json").unwrap();
    let records = build_trade_records(&clean_messages);

    for record in records {
        if let Some(first_outcome) = &record.first_outcome {
            match first_outcome {
                trading_analytics::types::Outcome::TP1 => {
                    assert!(
                        record.tp1_at.is_some(),
                        "TP1 outcome must have tp1_at set for {} {} {}",
                        record.symbol,
                        record.timeframe,
                        record.entry
                    );
                }
                trading_analytics::types::Outcome::TP2 => {
                    assert!(
                        record.tp2_at.is_some(),
                        "TP2 outcome must have tp2_at set for {} {} {}",
                        record.symbol,
                        record.timeframe,
                        record.entry
                    );
                }
                trading_analytics::types::Outcome::TP3 => {
                    assert!(
                        record.tp3_at.is_some(),
                        "TP3 outcome must have tp3_at set for {} {} {}",
                        record.symbol,
                        record.timeframe,
                        record.entry
                    );
                }
                trading_analytics::types::Outcome::SL => {
                    assert!(
                        record.sl_at.is_some(),
                        "SL outcome must have sl_at set for {} {} {}",
                        record.symbol,
                        record.timeframe,
                        record.entry
                    );
                }
            }
        }
    }
}

#[test]
fn take_profit_timestamps_are_monotonic_when_present() {
    let clean_messages = load_clean_messages_from_file("result.json").unwrap();
    let records = build_trade_records(&clean_messages);

    for record in records {
        if let (Some(tp1), Some(tp2)) = (record.tp1_at, record.tp2_at) {
            assert!(
                tp1 <= tp2,
                "tp1_at should be <= tp2_at for {} {} {}",
                record.symbol,
                record.timeframe,
                record.entry
            );
        }

        if let (Some(tp2), Some(tp3)) = (record.tp2_at, record.tp3_at) {
            assert!(
                tp2 <= tp3,
                "tp2_at should be <= tp3_at for {} {} {}",
                record.symbol,
                record.timeframe,
                record.entry
            );
        }

        if let (Some(tp1), Some(tp3)) = (record.tp1_at, record.tp3_at) {
            assert!(
                tp1 <= tp3,
                "tp1_at should be <= tp3_at for {} {} {}",
                record.symbol,
                record.timeframe,
                record.entry
            );
        }
    }
}

#[test]
fn output_is_sorted_by_opened_at_symbol_timeframe_and_entry() {
    let clean_messages = load_clean_messages_from_file("result.json").unwrap();
    let records = build_trade_records(&clean_messages);

    for pair in records.windows(2) {
        let a = &pair[0];
        let b = &pair[1];

        let a_key = (&a.opened_at, &a.symbol, &a.timeframe, &a.entry);
        let b_key = (&b.opened_at, &b.symbol, &b.timeframe, &b.entry);

        assert!(
            a_key <= b_key,
            "records are not sorted correctly:\nleft:  {:?} {:?} {:?} {:?}\nright: {:?} {:?} {:?} {:?}",
            a.opened_at,
            a.symbol,
            a.timeframe,
            a.entry,
            b.opened_at,
            b.symbol,
            b.timeframe,
            b.entry
        );
    }
}

#[test]
fn records_have_unique_signal_keys() {
    let clean_messages = load_clean_messages_from_file("result.json").unwrap();
    let records = build_trade_records(&clean_messages);

    let mut seen = std::collections::HashSet::new();

    for record in records {
        let inserted = seen.insert((
            record.symbol.clone(),
            record.timeframe.clone(),
            record.entry.clone(),
        ));

        assert!(
            inserted,
            "duplicate trade key found: {} {} {}",
            record.symbol,
            record.timeframe,
            record.entry
        );
    }
}