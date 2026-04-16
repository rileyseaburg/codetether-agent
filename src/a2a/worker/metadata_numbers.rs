//! Numeric metadata parsers for worker task settings.
//!
//! These helpers accept JSON numbers and numeric strings so queue payloads stay
//! flexible without spreading parsing logic through the worker.
//!
//! # Examples
//!
//! ```ignore
//! let timeout = metadata_u64(&metadata, &["timeout_secs"]);
//! ```

use super::metadata_lookup::metadata_lookup;

/// Parses an unsigned size value from metadata.
///
/// Non-negative integers are accepted from JSON numbers or strings.
///
/// # Examples
///
/// ```ignore
/// let top = metadata_usize(&metadata, &["top"]).unwrap_or(3);
/// ```
pub(super) fn metadata_usize(
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

/// Parses an unsigned 64-bit value from metadata.
///
/// Queue payloads may supply this as either a JSON number or numeric string.
///
/// # Examples
///
/// ```ignore
/// let timeout = metadata_u64(&metadata, &["timeout_secs"]);
/// ```
pub(super) fn metadata_u64(
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

/// Parses a floating-point value from metadata.
///
/// Queue payloads may send floating-point values as JSON numbers or strings.
///
/// # Examples
///
/// ```ignore
/// let min_alignment = metadata_f64(&metadata, &["moonshot_min_alignment"]);
/// ```
pub(super) fn metadata_f64(
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

fn parse_keys<T>(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
    parse: impl Fn(&serde_json::Value) -> Option<T>,
) -> Option<T> {
    keys.iter()
        .find_map(|key| metadata_lookup(metadata, key).and_then(&parse))
}
