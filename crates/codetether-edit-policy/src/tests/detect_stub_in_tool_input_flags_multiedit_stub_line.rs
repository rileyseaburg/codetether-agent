//! Original edit-input policy regression case.
use crate::detect_stub_in_tool_input;
use serde_json::json;

#[test]
fn detect_stub_in_tool_input_flags_multiedit_stub_line() {
    let args = json!({
        "edits": [
            {
                "file_path": "src/a.ts",
                "old_string": "x",
                "new_string": "throw new Error(\"Not implemented\")"
            }
        ]
    });
    let reason = detect_stub_in_tool_input("multiedit", &args);
    assert!(reason.is_some());
}
