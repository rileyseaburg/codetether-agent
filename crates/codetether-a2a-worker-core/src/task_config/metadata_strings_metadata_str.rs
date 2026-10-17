//! metadata str metadata parsing.
use super::*;

/// Parses a non-empty string value from metadata.
///
/// The first present key wins after whitespace trimming.
///
/// # Examples
///
/// ```ignore
/// let title = metadata_str(&metadata, &["title"]);
/// ```
pub fn metadata_str(
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
