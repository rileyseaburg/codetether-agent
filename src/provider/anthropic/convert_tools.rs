//! Tool definition conversion for Anthropic-compatible requests.
//!
//! Anthropic's Messages API accepts tool declarations as JSON objects containing
//! a tool name, natural-language description, and JSON Schema input definition.
//! This module translates the crate's provider-neutral [`ToolDefinition`] type
//! into that Anthropic `tools` array format.
//!
//! When prompt caching is enabled, the conversion marks the last tool definition
//! with Anthropic's ephemeral cache-control annotation. The caller is
//! responsible for deciding whether prompt caching is enabled for the current
//! provider configuration.

use serde_json::{Value, json};

use crate::provider::ToolDefinition;

/// Convert generic tool schemas into Anthropic `tools` entries.
///
/// # Arguments
///
/// * `tools` - Provider-neutral tool definitions to expose to the model.
/// * `enable_cache` - Whether to add Anthropic ephemeral cache-control metadata
///   to the final converted tool entry.
///
/// # Returns
///
/// A vector of JSON tool declarations suitable for the Anthropic Messages API.
/// The vector is empty when no tool definitions are supplied.
///
/// # Side Effects
///
/// This function has no external side effects. It only mutates the local
/// converted vector to add cache-control metadata when requested.
pub(crate) fn tools(tools: &[ToolDefinition], enable_cache: bool) -> Vec<Value> {
    let mut converted: Vec<Value> = tools.iter().map(tool).collect();
    if enable_cache && let Some(last_tool) = converted.last_mut() {
        super::cache::add_ephemeral_cache_control(last_tool);
    }
    converted
}

/// Convert one provider-neutral tool definition into an Anthropic tool object.
///
/// The `parameters` field is passed through as the Anthropic `input_schema`
/// value, preserving the caller's JSON Schema exactly.
///
/// # Arguments
///
/// * `t` - Tool definition to convert.
///
/// # Returns
///
/// A JSON object containing the Anthropic fields `"name"`, `"description"`,
/// and `"input_schema"`.
fn tool(t: &ToolDefinition) -> Value {
    json!({"name": t.name, "description": t.description, "input_schema": t.parameters})
}
