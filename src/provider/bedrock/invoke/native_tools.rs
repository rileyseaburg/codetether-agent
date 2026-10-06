//! `tools` declaration for native Anthropic Messages bodies (InvokeModel).
//!
//! Declares the request's tools, or — for tool-less requests such as the TUI
//! `/ask` side question that replay tool history — minimal specs for every
//! tool name referenced by `tool_use` blocks. Anthropic rejects bodies that
//! carry `tool_use`/`tool_result` blocks without a `tools` array.

use super::invoke_convert::convert_tools_native;
use crate::provider::ToolDefinition;
use serde_json::{Value, json};
use std::collections::BTreeSet;

/// Name declared when history only holds `tool_result` blocks.
const DEFAULT_TOOL: &str = "history_tool";

/// Set `body["tools"]` from `tools`, or from history when `tools` is empty.
pub(super) fn apply(body: &mut Value, tools: &[ToolDefinition]) {
    let declared = convert_tools_native(tools);
    if !declared.is_empty() {
        body["tools"] = Value::Array(declared);
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
        match block["type"].as_str() {
            Some("tool_use") => {
                any = true;
                if let Some(name) = block["name"].as_str() {
                    names.insert(name.to_string());
                }
            }
            Some("tool_result") => any = true,
            _ => {}
        }
    }
    if !any {
        return;
    }
    if names.is_empty() {
        names.insert(DEFAULT_TOOL.to_string());
    }
    body["tools"] = names.into_iter().map(spec).collect();
}

fn spec(name: String) -> Value {
    json!({
        "name": name,
        "description": "Tool referenced by prior conversation history.",
        "input_schema": {"type": "object"}
    })
}
