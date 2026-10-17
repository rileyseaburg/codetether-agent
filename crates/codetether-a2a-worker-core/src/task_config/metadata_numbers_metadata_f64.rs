//! metadata f64 metadata parsing.
use super::*;

/// Parses a floating-point value from metadata.
///
/// Queue payloads may send floating-point values as JSON numbers or strings.
///
/// # Examples
///
/// ```ignore
/// let min_alignment = metadata_f64(&metadata, &["moonshot_min_alignment"]);
/// ```
pub fn metadata_f64(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<f64> {
    parse_keys(metadata, keys, |value| {
        value
            .as_f64()
            .or_else(|| value.as_i64().map(|value| value as f64))
            .or_else(|| value.as_u64().map(|value| value as f64))
            .or_else(|| value.as_str()?.trim().parse::<f64>().ok())
    })
}
