//! Core coding-tool catalog and deterministic schema ordering.

use crate::provider::ToolDefinition;

const CODING_TOOL_IDS: &[&str] = &[
    "agent",
    "apply_patch",
    "browserctl",
    "codesearch",
    "computer_use",
    "exec_command",
    "glob",
    "grep",
    "image",
    "image_gen",
    "list",
    "lsp",
    "read",
    "session_task",
    "skill",
    "webfetch",
    "websearch",
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
