//! Bound retained text while preserving UTF-8 and diagnostic markers.
use codetether_util::truncate::truncate_bytes_safe;

/// Bound text to a byte budget with the existing diagnostic marker.
/// # Arguments
/// Input text, byte budget and marker label.
/// # Returns
/// Owned bounded UTF-8 text.
/// # Examples
/// ```
/// assert_eq!(codetether_view_retention::bounded("abc", 3, "text"), "abc");
/// ```
pub fn bounded(input: &str, max_bytes: usize, label: &str) -> String {
    if input.len() <= max_bytes {
        return input.to_string();
    }
    let marker = format!("\n\n[truncated: {label}]");
    if marker.len() >= max_bytes {
        return truncate_bytes_safe(input, max_bytes).to_string();
    }
    let prefix_max = max_bytes - marker.len();
    let mut out = truncate_bytes_safe(input, prefix_max).to_string();
    out.push_str(&marker);
    out
}
