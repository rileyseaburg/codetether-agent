//! CSS value normalization helpers for flex parsing.

use std::collections::HashMap;

pub fn val(s: &HashMap<String, String>, key: &str) -> Option<String> {
    s.get(key).map(|v| v.trim().to_ascii_lowercase())
}

pub fn parse_len(v: &str) -> Option<i64> {
    let v = v.trim();
    if v == "auto" {
        return None;
    }
    v.trim_end_matches("px")
        .parse::<f64>()
        .ok()
        .map(|n| n.round() as i64)
}
