use serde_json::Value;

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
