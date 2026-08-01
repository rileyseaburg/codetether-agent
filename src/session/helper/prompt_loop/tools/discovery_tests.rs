//! Regression tests for compact-profile schema promotion.

use super::promote;
use crate::provider::ToolDefinition;
use serde_json::json;

fn tool(name: &str) -> ToolDefinition {
    ToolDefinition {
        name: name.into(),
        description: format!("{name} description"),
        parameters: json!({"type": "object"}),
    }
}

#[test]
fn requested_hidden_tool_is_advertised_for_next_step() {
    let tools = vec![tool("read"), tool("memory")];
    let mut advertised = vec![tool("read")];

    promote(&tools, &mut advertised, &json!({"tool": "memory"}));

    let names: Vec<_> = advertised.iter().map(|tool| tool.name.as_str()).collect();
    assert_eq!(names, ["read", "memory"]);
    assert_eq!(advertised[1].parameters, json!({"type": "object"}));
}

#[test]
fn promotion_is_case_insensitive_and_idempotent() {
    let tools = vec![tool("session_recall")];
    let mut advertised = Vec::new();

    promote(&tools, &mut advertised, &json!({"tool": "SESSION_RECALL"}));
    promote(&tools, &mut advertised, &json!({"tool": "session_recall"}));

    assert_eq!(advertised.len(), 1);
    assert_eq!(advertised[0].name, "session_recall");
}

#[test]
fn unknown_tool_does_not_change_advertised_catalog() {
    let tools = vec![tool("read")];
    let mut advertised = vec![tool("read")];
    promote(&tools, &mut advertised, &json!({"tool": "invented"}));
    assert_eq!(advertised.len(), 1);
}
