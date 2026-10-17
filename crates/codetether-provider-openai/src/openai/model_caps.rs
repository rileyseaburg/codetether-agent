//! Capability and pricing detection helpers for `/models` entries.

use serde_json::Value;

/// Read a boolean capability flag, defaulting to `true` when absent.
pub(super) fn flag(entry: &Value, key: &str) -> bool {
    entry.get(key).and_then(Value::as_bool).unwrap_or(true)
}

/// Read a pricing field (USD per million tokens) from `/pricing/<key>`.
pub(super) fn price(entry: &Value, key: &str) -> Option<f64> {
    entry
        .pointer(&format!("/pricing/{key}"))
        .and_then(Value::as_f64)
}

/// Detect vision support from an explicit flag or `input_modalities`.
pub(super) fn detect_vision(entry: &Value) -> bool {
    entry
        .get("supports_vision")
        .and_then(Value::as_bool)
        .or_else(|| {
            entry
                .get("input_modalities")
                .and_then(Value::as_array)
                .map(|modalities| {
                    modalities.iter().any(|modality| {
                        modality
                            .as_str()
                            .is_some_and(|modality| modality.eq_ignore_ascii_case("image"))
                    })
                })
        })
        .unwrap_or(false)
}
