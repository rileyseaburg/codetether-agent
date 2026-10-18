//! Original edit-input policy regression case.
use crate::detect_stub_in_tool_input;
use serde_json::json;

#[test]
fn detect_stub_in_tool_input_flags_write_placeholder_content() {
    let args = json!({
        "path": "src/demo.ts",
        "content": "export function run(){ return \"Fallback prompt\"; } // Placeholder"
    });
    let reason = detect_stub_in_tool_input("write", &args);
    assert!(reason.is_some());
}
