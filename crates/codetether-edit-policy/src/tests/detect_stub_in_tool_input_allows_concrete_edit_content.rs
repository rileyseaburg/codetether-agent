//! Original edit-input policy regression case.
use crate::detect_stub_in_tool_input;
use serde_json::json;

#[test]
fn detect_stub_in_tool_input_allows_concrete_edit_content() {
    let args = json!({
        "old_string": "return a + b;",
        "new_string": "return sanitize(a) + sanitize(b);"
    });
    let reason = detect_stub_in_tool_input("edit", &args);
    assert!(reason.is_none());
}
