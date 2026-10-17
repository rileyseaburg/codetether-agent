//! Execute rlm_grep against normalized JSON arguments.
use super::super::RlmToolResult;
use super::super::output::clamp_tool_output;
use crate::repl::RlmRepl;
use serde_json::Value;
pub(super) fn execute(args: &Value, repl: &mut RlmRepl) -> Option<RlmToolResult> {
    let pattern = args.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
    let matches = repl.grep(pattern);
    let output = matches
        .iter()
        .map(|(i, line)| format!("{}:{}", i, line))
        .collect::<Vec<_>>()
        .join("\n");
    if output.is_empty() {
        Some(RlmToolResult::Output("(no matches)".to_string()))
    } else {
        Some(RlmToolResult::Output(clamp_tool_output(output, "rlm_grep")))
    }
}
