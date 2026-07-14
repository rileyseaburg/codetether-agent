//! Detection of direct and batched prior-context requests.

use serde_json::Value;

/// Tool IDs capable of consulting data outside the active conversation.
pub(super) const PRIOR_CONTEXT_TOOLS: [&str; 4] =
    ["session_recall", "context_browse", "memory", "search"];

/// Detect direct or batched calls that can consult prior context.
pub(super) fn requests_prior_context(name: &str, args: &Value) -> bool {
    if PRIOR_CONTEXT_TOOLS.contains(&name)
        || super::delegation::crosses_unenforced_boundary(name, args)
    {
        return true;
    }
    if name != "batch" {
        return false;
    }
    args.get("calls")
        .and_then(Value::as_array)
        .is_some_and(|calls| calls.iter().any(batch_call_requests_prior_context))
}

fn batch_call_requests_prior_context(call: &Value) -> bool {
    let name = call
        .get("tool")
        .or_else(|| call.get("name"))
        .and_then(Value::as_str)
        .unwrap_or_default();
    let args = call
        .get("args")
        .or_else(|| call.get("arguments"))
        .or_else(|| call.get("params"))
        .unwrap_or(&Value::Null);
    name == "agent" || requests_prior_context(name, args)
}
