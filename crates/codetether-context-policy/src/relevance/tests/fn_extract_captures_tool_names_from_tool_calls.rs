//! extract captures tool names from tool calls policy.
use super::*;

#[test]
pub(super) fn extract_captures_tool_names_from_tool_calls() {
    let meta = extract(&tool_call("Shell"));
    assert_eq!(meta.tools, vec!["Shell".to_string()]);
}
