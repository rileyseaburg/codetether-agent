//! UTF-8-safe retained-message truncation and marker formatting.
use codetether_util::truncate::truncate_bytes_safe;

pub(super) fn bounded(input: &str, max_bytes: usize, label: &str) -> String {
    if input.len() <= max_bytes {
        return input.to_string();
    }
    let marker = format!("\n\n[truncated in TUI retained message: {label}]");
    let content_bytes = max_bytes.saturating_sub(marker.len());
    let mut bounded = truncate_bytes_safe(input, content_bytes).to_string();
    bounded.push_str(&marker);
    bounded
}
