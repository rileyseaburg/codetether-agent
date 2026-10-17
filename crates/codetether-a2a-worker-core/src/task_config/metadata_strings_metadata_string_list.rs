//! metadata string list metadata parsing.
use super::*;

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
pub fn metadata_string_list(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Vec<String> {
    keys.iter()
        .find_map(|key| parse_value(metadata_lookup(metadata, key)?))
        .unwrap_or_default()
}
