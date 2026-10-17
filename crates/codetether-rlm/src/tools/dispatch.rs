//! Parse tool arguments and route recognized operation names.
use super::RlmToolResult;
use crate::repl::RlmRepl;
mod ast_query;
mod count;
mod final_answer;
mod grep;
mod head;
mod llm_query;
mod slice;
mod tail;
/// Dispatch a structured tool call against the in-memory REPL.
///
/// # Arguments
/// * `name` - Tool name to dispatch.
/// * `arguments` - JSON arguments; invalid JSON retains default-value behavior.
/// * `repl` - Context and variables used by the operation.
/// # Returns
/// Operation output or a final answer; None for unknown tools. Sub-queries
/// return a JSON sentinel for the caller to handle asynchronously.
/// # Examples
/// ```
/// use codetether_rlm::{tools::dispatch_tool_call, repl::{RlmRepl, ReplRuntime}};
/// let mut repl = RlmRepl::new("text".into(), ReplRuntime::Rust);
/// assert!(dispatch_tool_call("unknown", "{}", &mut repl).is_none());
/// ```
pub fn dispatch_tool_call(
    name: &str,
    arguments: &str,
    repl: &mut RlmRepl,
) -> Option<RlmToolResult> {
    let args: serde_json::Value = serde_json::from_str(arguments).unwrap_or_default();
    match name {
        "rlm_head" => head::execute(&args, repl),
        "rlm_tail" => tail::execute(&args, repl),
        "rlm_grep" => grep::execute(&args, repl),
        "rlm_count" => count::execute(&args, repl),
        "rlm_slice" => slice::execute(&args, repl),
        "rlm_llm_query" => llm_query::execute(&args),
        "rlm_final" => final_answer::execute(&args),
        "rlm_ast_query" => ast_query::execute(&args, repl),
        _ => None,
    }
}
