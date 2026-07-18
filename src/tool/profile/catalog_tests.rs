//! Tests for the compact tool catalog.

use super::retain_coding_tools;
use crate::provider::ToolDefinition;
use serde_json::json;

fn definition(name: &str) -> ToolDefinition {
    ToolDefinition {
        name: name.to_string(),
        description: String::new(),
        parameters: json!({}),
    }
}

#[test]
fn compact_catalog_keeps_core_and_mcp_tools_without_edit_aliases() {
    let names = [
        "write",
        "mcp:issues",
        "apply_patch",
        "edit",
        "bash",
        "exec_command",
        "close_agent",
        "resume_agent",
        "send_input",
        "send_message",
        "write_stdin",
    ];
    let retained = retain_coding_tools(names.into_iter().map(definition).collect());
    let retained: Vec<_> = retained.iter().map(|tool| tool.name.as_str()).collect();

    assert_eq!(
        retained,
        [
            "mcp:issues",
            "apply_patch",
            "exec_command",
            "close_agent",
            "resume_agent",
            "send_input",
            "send_message",
            "write_stdin"
        ]
    );
}

#[path = "catalog_sort_tests.rs"]
mod sort_tests;
