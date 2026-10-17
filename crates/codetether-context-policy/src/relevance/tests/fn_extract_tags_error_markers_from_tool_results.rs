//! extract tags error markers from tool results policy.
use super::*;

#[test]
pub(super) fn extract_tags_error_markers_from_tool_results() {
    let meta = extract(&tool_result(
        "Error: file not found\n  panicked at main.rs:12",
    ));
    assert!(meta.error_classes.contains(&"error".to_string()));
    assert!(meta.error_classes.contains(&"panicked".to_string()));
}
