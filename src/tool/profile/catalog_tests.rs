//! Tests for the compact tool catalog.

use super::{retain_coding_tools, sort};
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
        "write_stdin",
    ];
    let retained = retain_coding_tools(names.into_iter().map(definition).collect());
    let retained: Vec<_> = retained.iter().map(|tool| tool.name.as_str()).collect();

    assert_eq!(
        retained,
        ["mcp:issues", "apply_patch", "exec_command", "write_stdin"]
    );
}

#[test]
fn schema_order_is_deterministic() {
    let definitions = ["read", "apply_patch", "bash"]
        .into_iter()
        .map(definition)
        .collect();
    let names: Vec<_> = sort(definitions)
        .into_iter()
        .map(|tool| tool.name)
        .collect();

    assert_eq!(names, ["apply_patch", "bash", "read"]);
}
