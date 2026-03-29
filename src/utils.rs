/// Returns `true` for symbols that should be excluded from all analysis.
pub fn is_banned_symbol(symbol: &str) -> bool {
    matches!(symbol, "ASTERUSDT" | "ASTER")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banned_symbols() {
        assert!(is_banned_symbol("ASTERUSDT"));
        assert!(is_banned_symbol("ASTER"));
    }

    #[test]
    fn test_non_banned_symbols() {
        assert!(!is_banned_symbol("BTCUSDT"));
        assert!(!is_banned_symbol("ETHUSDT"));
        assert!(!is_banned_symbol("ADAUSDT"));
    }
}