//! Original edit-input policy regression case.
use crate::normalize_tool_call_for_execution;
use serde_json::json;

#[test]
fn normalize_tool_call_for_execution_maps_multiedit_aliases() {
    let args = json!({
        "changes": [
            {
                "filePath": "src/main.rs",
                "oldString": "old",
                "newString": "new"
            }
        ]
    });
    let (name, normalized) = normalize_tool_call_for_execution("multiedit", &args);
    assert_eq!(name, "multiedit");
    assert_eq!(normalized["edits"][0]["file"], "src/main.rs");
    assert_eq!(normalized["edits"][0]["old_string"], "old");
    assert_eq!(normalized["edits"][0]["new_string"], "new");
}
