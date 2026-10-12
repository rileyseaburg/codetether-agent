//! Conservative normalization of historical tool arguments to JSON strings.
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{Value, json};

pub(crate) fn normalize(arguments: &str) -> String {
    if let Ok(parsed) = serde_json::from_str::<Value>(arguments) {
        return if parsed.is_object() {
            parsed.to_string()
        } else {
            json!({"input": parsed}).to_string()
        };
    }
    if let Some(salvaged) = salvage(arguments) {
        return salvaged.to_string();
    }
    json!({"input": arguments}).to_string()
}

fn salvage(arguments: &str) -> Option<Value> {
    let trimmed = arguments.trim();
    if !trimmed.starts_with('{') {
        return None;
    }
    static PAIR: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"(?s)\"(?P<k>[^\"\\]*(?:\\.[^\"\\]*)*)\"\s*:\s*(?P<v>\"(?:\\.|[^\"])*\"|true|false|null|-?\d+(?:\.\d+)?(?:[eE][+-]?\d+)?)"#)
            .expect("invalid tool argument regex")
    });
    let mut map = serde_json::Map::new();
    for caps in PAIR.captures_iter(trimmed) {
        if let Ok(value) = serde_json::from_str::<Value>(caps.name("v")?.as_str()) {
            map.insert(caps.name("k")?.as_str().into(), value);
        }
    }
    if map.is_empty() {
        None
    } else {
        Some(Value::Object(map))
    }
}
