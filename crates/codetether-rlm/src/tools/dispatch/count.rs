//! Execute rlm_count against normalized JSON arguments.
use super::super::RlmToolResult;
use crate::repl::RlmRepl;
use serde_json::Value;
pub(super) fn execute(args: &Value, repl: &mut RlmRepl) -> Option<RlmToolResult> {
    let pattern = args.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
    let count = repl.count(pattern);
    Some(RlmToolResult::Output(count.to_string()))
}
