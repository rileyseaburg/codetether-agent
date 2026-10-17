//! Execute rlm_final against normalized JSON arguments.
use super::super::RlmToolResult;
use serde_json::Value;
pub(super) fn execute(args: &Value) -> Option<RlmToolResult> {
    if let Some(payload) = args.get("payload") {
        if let Some(text_payload) = payload.as_str() {
            return Some(RlmToolResult::Final(text_payload.to_string()));
        }
        return Some(RlmToolResult::Final(payload.to_string()));
    }
    let answer = args
        .get("answer")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    Some(RlmToolResult::Final(answer))
}
