//! Original prose recovery regression case.
use super::support::bash_tool;
use crate::extract_tool_call_from_prose;

#[test]
fn returns_none_when_prose_mentions_no_command() {
    let tools = bash_tool();
    assert!(extract_tool_call_from_prose("I think bash could help here.", &tools).is_none());
}
