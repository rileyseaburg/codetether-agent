//! metadata usize metadata parsing.
use super::*;

/// Parses an unsigned size value from metadata.
///
/// Non-negative integers are accepted from JSON numbers or strings.
///
/// # Examples
///
/// ```ignore
/// let top = metadata_usize(&metadata, &["top"]).unwrap_or(3);
/// ```
pub fn metadata_usize(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<usize> {
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
            .and_then(|value| usize::try_from(value).ok())
    })
}
