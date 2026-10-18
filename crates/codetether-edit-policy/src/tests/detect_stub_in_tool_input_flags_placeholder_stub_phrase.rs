//! Original edit-input policy regression case.
use crate::detect_stub_in_tool_input;
use serde_json::json;

#[test]
fn detect_stub_in_tool_input_flags_placeholder_stub_phrase() {
    let args = json!({
        "path": "src/demo.ts",
        "content": "// Placeholder implementation\nexport const run = () => null;"
    });
    let reason = detect_stub_in_tool_input("write", &args);
    assert!(reason.is_some());
}
