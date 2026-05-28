//! Auto-process context construction for the RLM tool.

use super::RlmTool;
use crate::rlm::router::AutoProcessContext;
use serde_json::{Value, json};
use std::sync::Arc;

pub(super) fn action(args: &Value) -> anyhow::Result<&str> {
    args["action"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("action is required"))
}

pub(super) fn paths(args: &Value) -> Vec<&str> {
    args["paths"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default()
}

pub(super) fn auto<'a>(
    tool: &RlmTool,
    action: &'a str,
    query: &str,
    paths: &[&str],
) -> AutoProcessContext<'a> {
    let effective_query = if query.is_empty() {
        format!("Summarize the content from: {paths:?}")
    } else {
        query.to_string()
    };
    AutoProcessContext {
        tool_id: action,
        tool_args: json!({ "query": effective_query, "paths": paths }),
        session_id: "rlm-tool",
        abort: None,
        on_progress: None,
        provider: Arc::clone(&tool.provider),
        model: tool.model.clone(),
        bus: None,
        trace_id: Some(uuid::Uuid::new_v4()),
        subcall_provider: None,
        subcall_model: None,
    }
}
