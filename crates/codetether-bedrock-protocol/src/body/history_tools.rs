//! Synthesize `toolConfig` for tool-less requests that replay tool history.
//!
//! Bedrock rejects any request whose messages contain `toolUse` or
//! `toolResult` blocks unless `toolConfig` is defined. Callers such as the
//! TUI `/ask` side question deliberately send `tools: []` while replaying a
//! session history full of tool calls, so we declare minimal specs for
//! every tool name referenced in that history.

use serde_json::{Value, json};
use std::collections::BTreeSet;

/// Name declared when history only holds `toolResult` blocks.
const DEFAULT_TOOL: &str = "history_tool";

/// Ensure `body["toolConfig"]` exists when its messages reference tools.
///
/// Leaves bodies that already declare tools, or contain no tool blocks,
/// untouched.
pub(super) fn ensure(body: &mut Value) {
    if body.get("toolConfig").is_some() {
        return;
    }
    let Some(messages) = body["messages"].as_array() else {
        return;
    };
    let (mut names, mut any) = (BTreeSet::new(), false);
    for block in messages
        .iter()
        .filter_map(|m| m["content"].as_array())
        .flatten()
    {
        if let Some(name) = block["toolUse"]["name"].as_str() {
            names.insert(name.to_string());
        }
        any |= block.get("toolUse").is_some() || block.get("toolResult").is_some();
    }
    if !any {
        return;
    }
    if names.is_empty() {
        names.insert(DEFAULT_TOOL.to_string());
    }
    let tools: Vec<Value> = names.into_iter().map(spec).collect();
    body["toolConfig"] = json!({ "tools": tools });
}

fn spec(name: String) -> Value {
    json!({"toolSpec": {
        "name": name,
        "description": "Tool referenced by prior conversation history.",
        "inputSchema": {"json": {"type": "object"}}
    }})
}
