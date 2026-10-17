//! metadata u64 metadata parsing.
use super::*;

/// Parses an unsigned 64-bit value from metadata.
///
/// Queue payloads may supply this as either a JSON number or numeric string.
///
/// # Examples
///
/// ```ignore
/// let timeout = metadata_u64(&metadata, &["timeout_secs"]);
/// ```
pub fn metadata_u64(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<u64> {
    parse_keys(metadata, keys, |value| {
        value
            .as_u64()
            .or_else(|| {
                value
                    .as_i64()
                    .filter(|value| *value >= 0)
                    .map(|value| value as u64)
            })
            .or_else(|| value.as_str()?.trim().parse::<u64>().ok())
    })
}
