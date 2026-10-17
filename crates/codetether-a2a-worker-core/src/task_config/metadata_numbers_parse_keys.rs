//! parse keys metadata parsing.
use super::*;

pub(super) fn parse_keys<T>(
    metadata: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
    parse: impl Fn(&serde_json::Value) -> Option<T>,
) -> Option<T> {
    keys.iter()
        .find_map(|key| metadata_lookup(metadata, key).and_then(&parse))
}
