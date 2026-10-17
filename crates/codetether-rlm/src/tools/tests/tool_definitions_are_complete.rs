//! Existing tool_definitions_are_complete contract check.
use super::*;

#[test]
fn tool_definitions_are_complete() {
    let defs = rlm_tool_definitions();
    assert_eq!(defs.len(), 8);
    let names: Vec<&str> = defs.iter().map(|d| d.name.as_str()).collect();
    assert!(names.contains(&"rlm_head"));
    assert!(names.contains(&"rlm_tail"));
    assert!(names.contains(&"rlm_grep"));
    assert!(names.contains(&"rlm_count"));
    assert!(names.contains(&"rlm_slice"));
    assert!(names.contains(&"rlm_llm_query"));
    assert!(names.contains(&"rlm_final"));
    assert!(names.contains(&"rlm_ast_query"));
}
