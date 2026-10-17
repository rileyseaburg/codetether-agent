//! json to prost struct conversion.
use super::*;

/// Convert a `serde_json::Value` to a `prost_types::Struct`.
pub fn json_to_prost_struct(v: &serde_json::Value) -> prost_types::Struct {
    match v {
        serde_json::Value::Object(map) => prost_types::Struct {
            fields: map
                .iter()
                .map(|(k, v)| (k.clone(), json_to_prost_value(v)))
                .collect(),
        },
        _ => prost_types::Struct::default(),
    }
}
