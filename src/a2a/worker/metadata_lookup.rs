//! Metadata lookup helpers for worker task routing.
//!
//! Metadata may be nested under routing, swarm, or forage sections. This
//! module centralizes that lookup order for reuse across task handlers.
//!
//! # Examples
//!
//! ```ignore
//! let value = metadata_lookup(&metadata, "model");
//! ```

/// Resolves a metadata key from the primary object or known nested sections.
///
/// Nested lookup supports backward-compatible payloads that route parameters
/// through `routing`, `swarm`, or `forage`.
///
/// # Examples
///
/// ```ignore
/// let value = metadata_lookup(&metadata, "agent_type");
/// ```
pub(super) fn metadata_lookup<'a>(
    metadata: &'a serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> Option<&'a serde_json::Value> {
    metadata
        .get(key)
        .or_else(|| nested_lookup(metadata, "routing", key))
        .or_else(|| nested_lookup(metadata, "swarm", key))
        .or_else(|| nested_lookup(metadata, "forage", key))
}

fn nested_lookup<'a>(
    metadata: &'a serde_json::Map<String, serde_json::Value>,
    scope: &str,
    key: &str,
) -> Option<&'a serde_json::Value> {
    metadata
        .get(scope)
        .and_then(serde_json::Value::as_object)
        .and_then(|object| object.get(key))
}
