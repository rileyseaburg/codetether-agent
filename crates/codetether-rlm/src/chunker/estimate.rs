//! Token estimation heuristics.

/// Estimate token count (roughly 2.8–4 chars per token depending on
/// whitespace density).
pub fn estimate_tokens(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }
    let len = text.len();
    let whitespace = text
        .as_bytes()
        .iter()
        .filter(|b| b.is_ascii_whitespace())
        .count();
    let ws_ratio = whitespace as f64 / len as f64;
    let chars_per_token = if ws_ratio < 0.05 {
        2.8
    } else if ws_ratio < 0.10 {
        3.2
    } else if ws_ratio < 0.20 {
        3.6
    } else {
        4.0
    };
    ((len as f64) / chars_per_token).ceil() as usize
}
