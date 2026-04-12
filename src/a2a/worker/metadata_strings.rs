//! String metadata parsers for worker task routing.
//!
//! These helpers keep trimming and list normalization out of task handlers so
//! queue metadata remains easy to consume.
//!
//! # Examples
//!
//! ```ignore
//! let mission = metadata_str(&metadata, &["moonshot"]);
//! ```

use super::metadata_lookup::metadata_lookup;

/// Parses a non-empty string value from metadata.
///
/// The first present key wins after whitespace trimming.
///
/// # Examples
///
/// ```ignore
/// let title = metadata_str(&metadata, &["title"]);
/// ```
pub(super) fn metadata_str(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<String> {
    keys.iter().find_map(|key| {
        metadata_lookup(metadata, key)
            .and_then(serde_json::Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string)
    })
}

/// Parses a list of strings from metadata.
///
/// Arrays are preserved, while comma- or newline-delimited strings are split
/// into trimmed items.
///
/// # Examples
///
/// ```ignore
/// let goals = metadata_string_list(&metadata, &["moonshots"]);
/// ```
pub(super) fn metadata_string_list(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Vec<String> {
    keys.iter()
        .find_map(|key| parse_value(metadata_lookup(metadata, key)?))
        .unwrap_or_default()
}

fn parse_value(value: &serde_json::Value) -> Option<Vec<String>> {
    value
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        })
        .filter(|items| !items.is_empty())
        .or_else(|| {
            value
                .as_str()
                .map(|items| {
                    items
                        .split(['\n', ','])
                        .map(str::trim)
                        .filter(|item| !item.is_empty())
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                })
                .filter(|items| !items.is_empty())
        })
}
