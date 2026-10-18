//! Original edit-input policy regression case.
use crate::normalize_tool_call_for_execution;
use serde_json::json;

#[test]
fn normalize_tool_call_for_execution_maps_advanced_edit_to_edit_shape() {
    let args = json!({
        "filePath": "src/lib.rs",
        "oldString": "old",
        "newString": "new"
    });
    let (name, normalized) = normalize_tool_call_for_execution("advanced_edit", &args);
    assert_eq!(name, "edit");
    assert_eq!(normalized["path"], "src/lib.rs");
    assert_eq!(normalized["old_string"], "old");
    assert_eq!(normalized["new_string"], "new");
}
