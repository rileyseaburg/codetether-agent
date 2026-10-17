//! prost struct to json conversion.
use super::*;

// ═══════════════════════════════════════════════════════════════════════
// prost_types::Struct ↔ serde_json::Value helpers
// ═══════════════════════════════════════════════════════════════════════

/// Convert a `prost_types::Struct` to a `serde_json::Value`.
pub fn prost_struct_to_json(s: &prost_types::Struct) -> serde_json::Value {
    let map: serde_json::Map<String, serde_json::Value> = s
        .fields
        .iter()
        .map(|(k, v)| (k.clone(), prost_value_to_json(v)))
        .collect();
    serde_json::Value::Object(map)
}
