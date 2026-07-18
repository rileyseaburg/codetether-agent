//! Core coding-tool catalog and deterministic schema ordering.

use crate::provider::ToolDefinition;

const CODING_TOOL_IDS: &[&str] = &[
    "agent",
    "apply_patch",
    "browserctl",
    "close_agent",
    "codesearch",
    "computer_use",
    "create_goal",
    "exec_command",
    "followup_task",
    "glob",
    "grep",
    "get_goal",
    "image",
    "image_gen",
    "interrupt_agent",
    "list",
    "list_agents",
    "lsp",
    "read",
    "resume_agent",
    "send_input",
    "send_message",
    "session_task",
    "skill",
    "spawn_agent",
    "webfetch",
    "websearch",
    "update_goal",
    "wait_agent",
    "write_stdin",
];

pub(super) fn retain_coding_tools(definitions: Vec<ToolDefinition>) -> Vec<ToolDefinition> {
    definitions
        .into_iter()
        .filter(|tool| CODING_TOOL_IDS.contains(&tool.name.as_str()) || is_mcp(&tool.name))
        .collect()
}

pub(super) fn sort(mut definitions: Vec<ToolDefinition>) -> Vec<ToolDefinition> {
    definitions.sort_unstable_by(|left, right| left.name.cmp(&right.name));
    definitions
}

fn is_mcp(name: &str) -> bool {
    name.starts_with("mcp:") || name.starts_with("mcp__")
}

#[cfg(test)]
#[path = "catalog_tests.rs"]
mod tests;
