//! Execute rlm_slice against normalized JSON arguments.
use super::super::RlmToolResult;
use super::super::output::clamp_tool_output;
use crate::repl::RlmRepl;
use serde_json::Value;
pub(super) fn execute(args: &Value, repl: &mut RlmRepl) -> Option<RlmToolResult> {
    let start = args.get("start").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let end = args.get("end").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
    let output = clamp_tool_output(repl.slice(start, end).to_string(), "rlm_slice");
    Some(RlmToolResult::Output(output))
}
