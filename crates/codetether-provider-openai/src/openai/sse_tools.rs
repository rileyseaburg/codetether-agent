//! Serialize provider-neutral function definitions for raw chat requests.
use codetether_provider_core::ToolDefinition;
use serde_json::{Value, json};

pub(super) fn tools_json(tools: &[ToolDefinition]) -> Vec<Value> {
    tools
        .iter()
        .map(|tool| {
            json!({
                "type": "function",
                "function": {
                    "name": tool.name,
                    "description": tool.description,
                    "parameters": tool.parameters,
                }
            })
        })
        .collect()
}
