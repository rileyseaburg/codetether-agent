//! Original edit-input policy regression case.
use crate::detect_stub_in_tool_input;
use serde_json::json;

#[test]
fn detect_stub_in_tool_input_allows_html_placeholder_attribute() {
    let args = json!({
        "old_string": "<input type=\"text\" />",
        "new_string": "<input type=\"text\" placeholder=\"Search users\" />"
    });
    let reason = detect_stub_in_tool_input("edit", &args);
    assert!(reason.is_none());
}
