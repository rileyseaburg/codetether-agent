//! Execute rlm_llm_query against normalized JSON arguments.
use super::super::RlmToolResult;
use serde_json::Value;
pub(super) fn execute(args: &Value) -> Option<RlmToolResult> {
    // The llm_query tool requires async provider calls — return a
    // sentinel so the caller knows to handle it specially.
    let query = args
        .get("query")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let context_slice = args
        .get("context_slice")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    // Encode the query + optional slice as JSON so the caller can
    // destructure it.
    let payload = serde_json::json!({
        "__rlm_llm_query": true,
        "query": query,
        "context_slice": context_slice,
    });
    Some(RlmToolResult::Output(payload.to_string()))
}
