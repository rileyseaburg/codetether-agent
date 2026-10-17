//! Application tool-schema encoding.
use super::ZaiProvider;
use crate::ToolDefinition;
use serde_json::{Value, json};
impl ZaiProvider {
    pub(super) fn convert_tools(tools: &[ToolDefinition]) -> Vec<Value> {
        tools
            .iter()
            .map(|t| {
                json!({
                    "type": "function",
                    "function": {
                        "name": t.name,
                        "description": t.description,
                        "parameters": t.parameters
                    }
                })
            })
            .collect()
    }
}
