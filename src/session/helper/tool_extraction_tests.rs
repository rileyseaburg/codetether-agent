//! Tests for prose-based tool-call extraction.

use super::tool_extraction::extract_tool_call_from_prose;
use crate::provider::ToolDefinition;

fn bash_tool() -> Vec<ToolDefinition> {
    vec![ToolDefinition {
        name: "bash".to_string(),
        description: "shell".to_string(),
        parameters: serde_json::json!({"type": "object"}),
    }]
}

#[test]
fn extracts_backticked_command_from_prose() {
    let tools = bash_tool();
    let (_id, name, args) =
        extract_tool_call_from_prose("I'll use bash to run `ls -la` now.", &tools)
            .expect("should extract a synthetic bash call");
    assert_eq!(name, "bash");
    assert_eq!(args["command"], "ls -la");
}

#[test]
fn extracts_command_after_run_verb_without_backticks() {
    let tools = bash_tool();
    let (_id, name, args) = extract_tool_call_from_prose("Let me use bash and run ls -la", &tools)
        .expect("should extract a synthetic bash call");
    assert_eq!(name, "bash");
    assert_eq!(args["command"], "ls -la");
}

#[test]
fn returns_none_when_bash_tool_unavailable() {
    let tools = vec![ToolDefinition {
        name: "read".to_string(),
        description: "read".to_string(),
        parameters: serde_json::json!({"type": "object"}),
    }];
    assert!(extract_tool_call_from_prose("I'll use bash to run `ls`", &tools).is_none());
}

#[test]
fn returns_none_when_prose_mentions_no_command() {
    let tools = bash_tool();
    assert!(extract_tool_call_from_prose("I think bash could help here.", &tools).is_none());
}
