//! Boolean metadata parsing helpers.
//!
//! These helpers normalize queue flags from either JSON booleans or common
//! string spellings such as `yes` and `off`.
//!
//! # Examples
//!
//! ```ignore
//! let execute = metadata_bool(&metadata, &["execute"]);
//! ```

use super::metadata_lookup::metadata_lookup;

/// Parses a boolean value from queue metadata.
///
/// String forms such as `true`, `false`, `yes`, and `off` are accepted in
/// addition to native JSON booleans.
///
/// # Examples
///
/// ```ignore
/// let execute = metadata_bool(&metadata, &["execute"]).unwrap_or(false);
/// ```
pub(super) fn metadata_bool(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
) -> Option<bool> {
    keys.iter().find_map(|key| {
        let value = metadata_lookup(metadata, key)?;
        value.as_bool().or_else(
            || match value.as_str()?.trim().to_ascii_lowercase().as_str() {
                "1" | "true" | "yes" | "on" => Some(true),
                "0" | "false" | "no" | "off" => Some(false),
                _ => None,
            },
        )
    })
}
