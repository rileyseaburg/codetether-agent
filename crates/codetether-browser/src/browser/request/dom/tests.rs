//! DOM payload serialization preserves field names and explicit null scopes.
use super::{ClickTextRequest, FillRequest};
use serde_json::json;

#[test]
fn fill_preserves_null_frame_scope() {
    let value = FillRequest {
        selector: "#name".into(),
        value: "Alice".into(),
        frame_selector: None,
    };
    assert_eq!(
        serde_json::to_value(value).unwrap(),
        json!({"selector":"#name", "value":"Alice", "frame_selector":null})
    );
}

#[test]
fn text_target_preserves_optional_fields_and_selection_controls() {
    let value = ClickTextRequest {
        selector: None,
        frame_selector: Some("iframe".into()),
        text: "Submit".into(),
        timeout_ms: 20,
        exact: true,
        index: 2,
    };
    assert_eq!(
        serde_json::to_value(value).unwrap(),
        json!({
            "selector":null, "frame_selector":"iframe", "text":"Submit",
            "timeout_ms":20, "exact":true, "index":2,
        })
    );
}
