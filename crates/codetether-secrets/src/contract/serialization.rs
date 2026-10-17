//! serialization contract.
use super::{fixture::Fixture, response};
use serde_json::json;

#[test]
fn provider_debug_and_flattened_fields() {
    let value: crate::ProviderSecrets =
        serde_json::from_value(json!({"api_key":"fixture-private","custom":7})).unwrap();
    assert!(value.has_valid_api_key());
    assert_eq!(value.api_key_len(), Some(15));
    let debug = format!("{value:?}");
    assert!(!debug.contains("fixture-private"));
    assert!(debug.contains("REDACTED"));
    assert_eq!(serde_json::to_value(value).unwrap()["custom"], 7);
}
