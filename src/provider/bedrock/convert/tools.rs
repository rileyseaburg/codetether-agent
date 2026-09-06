//! Bedrock tool-definition serialization.
use crate::provider::ToolDefinition;
use serde_json::{Value, json};

/// Convert crate-internal [`ToolDefinition`]s into Bedrock `toolConfig.tools`
/// entries.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::bedrock::convert_tools;
/// use codetether_agent::provider::ToolDefinition;
/// use serde_json::json;
///
/// let t = vec![ToolDefinition {
///     name: "ls".into(),
///     description: "List files".into(),
///     parameters: json!({"type":"object"}),
/// }];
/// let out = convert_tools(&t);
/// assert_eq!(out[0]["toolSpec"]["description"], "List files");
/// ```
pub fn convert_tools(tools: &[ToolDefinition]) -> Vec<Value> {
    tools
        .iter()
        .map(|t| {
            json!({
                "toolSpec": {
                    "name": t.name,
                    "description": t.description,
                    "inputSchema": {
                        "json": t.parameters
                    }
                }
            })
        })
        .collect()
}
