//! Tests for the prompt-size-bounded discovery catalog.

use super::retain_discovery_tools;
use crate::provider::ToolDefinition;
use serde_json::json;

fn tool(name: &str) -> ToolDefinition {
    ToolDefinition {
        name: name.into(),
        description: String::new(),
        parameters: json!({}),
    }
}

#[test]
fn discovery_catalog_keeps_core_tools_and_hides_specialists() {
    let definitions = ["memory", "read", "exec_command", "session_task", "youtube"]
        .into_iter()
        .map(tool)
        .collect();

    let retained = retain_discovery_tools(definitions);
    let names: Vec<_> = retained.iter().map(|tool| tool.name.as_str()).collect();

    assert_eq!(names, ["read", "exec_command", "session_task"]);
}
