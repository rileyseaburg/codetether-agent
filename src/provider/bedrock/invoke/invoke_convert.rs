//! Native Anthropic Messages converters for the Bedrock InvokeModel path.
//!
//! The InvokeModel adapter (`invoke.rs`) sends a *native* Anthropic Messages
//! body (`anthropic_version`, `type`-tagged content blocks), not the Bedrock
//! Converse schema. The Converse converters emit `toolSpec` / `toolUse` /
//! `{"text":...}`, which the native API rejects with e.g.
//! `tools.0.custom.name: Field required`. This module produces the native
//! shape instead.

use crate::provider::ToolDefinition;
use serde_json::{Value, json};

/// Convert tool definitions into native Anthropic `tools` entries.
///
/// Native Anthropic requires `{name, description, input_schema}`, unlike the
/// Bedrock Converse `toolSpec` wrapper. Emitting the Converse shape here
/// triggers `tools.0.custom.name: Field required` on InvokeModel.
pub fn convert_tools_native(tools: &[ToolDefinition]) -> Vec<Value> {
    tools
        .iter()
        .map(|t| {
            json!({
                "name": t.name,
                "description": t.description,
                "input_schema": t.parameters,
            })
        })
        .collect()
}
