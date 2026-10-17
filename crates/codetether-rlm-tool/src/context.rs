//! Action parsing and RLM query arguments.
use serde_json::{Value, json};

pub(crate) fn action(args: &Value) -> anyhow::Result<&str> {
    args["action"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("action is required"))
}

pub(crate) fn paths(args: &Value) -> Vec<&str> {
    args["paths"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default()
}
pub(crate) fn args(query: &str, paths: &[&str]) -> Value {
    let effective_query = if query.is_empty() {
        format!("Summarize the content from: {paths:?}")
    } else {
        query.to_string()
    };

    json!({ "query": effective_query, "paths": paths })
}
