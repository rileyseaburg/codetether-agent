//! Execute rlm_tail against normalized JSON arguments.
use super::super::RlmToolResult;
use super::super::output::clamp_tool_output;
use crate::repl::RlmRepl;
use serde_json::Value;
pub(super) fn execute(args: &Value, repl: &mut RlmRepl) -> Option<RlmToolResult> {
    let n = args.get("n").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
    let output = clamp_tool_output(repl.tail(n).join("\n"), "rlm_tail");
    Some(RlmToolResult::Output(output))
}
