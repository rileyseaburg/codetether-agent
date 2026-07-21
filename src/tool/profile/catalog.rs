//! Core coding-tool catalog and deterministic schema ordering.

use crate::provider::ToolDefinition;

#[path = "catalog_ids.rs"]
mod ids;

pub(super) fn retain_coding_tools(definitions: Vec<ToolDefinition>) -> Vec<ToolDefinition> {
    definitions
        .into_iter()
        .filter(|tool| ids::CODING.contains(&tool.name.as_str()) || is_mcp(&tool.name))
        .collect()
}

pub(super) fn retain_mux_manager_tools(definitions: Vec<ToolDefinition>) -> Vec<ToolDefinition> {
    definitions
        .into_iter()
        .filter(|tool| ids::MUX_MANAGER.contains(&tool.name.as_str()))
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
