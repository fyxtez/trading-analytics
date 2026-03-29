pub mod parsing;
pub mod trades;
pub mod types;
pub mod utils;

use trading_analytics::{parsing::load_clean_messages_from_file, trades::build_trade_records};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clean_messages = load_clean_messages_from_file("result.json")?;
    let trades = build_trade_records(&clean_messages);

    println!("Loaded {} clean messages", clean_messages.len());
    println!("Built {} trades", trades.len());

    Ok(())
}
