//! Tool definition conversion for DeepSeek.

use crate::provider::ToolDefinition;
use serde_json::{Value, json};

pub(crate) fn tools(tools: &[ToolDefinition]) -> Vec<Value> {
    tools.iter().map(|t| json!({
        "type": "function",
        "function": {"name": t.name, "description": t.description, "parameters": t.parameters}
    })).collect()
}
