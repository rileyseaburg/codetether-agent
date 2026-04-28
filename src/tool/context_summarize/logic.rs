//! Summarize lookup logic for [`ContextSummarizeTool`].

use super::super::ToolResult;
use crate::session::Session;
use crate::session::index::types::{SummaryNode, SummaryRange};
use serde_json::Value;
/// Parse and validate the requested summary range.
pub fn parse_range(args: &Value) -> Result<(SummaryRange, usize), ToolResult> {
    let start = required_u64(args, "start")? as usize;
    let end = required_u64(args, "end")? as usize;
    let target = args["target_tokens"].as_u64().unwrap_or(512) as usize;
    let range = SummaryRange::new(start, end)
        .ok_or_else(|| ToolResult::error("Invalid range: start must be < end."))?;
    Ok((range, target))
}

fn required_u64(args: &Value, key: &str) -> Result<u64, ToolResult> {
    args.get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| ToolResult::error(&format!("`{key}` must be a non-negative integer.")))
}

/// Look up a cached summary from the session index.
pub fn lookup_cached(session: &Session, range: SummaryRange) -> Option<&SummaryNode> {
    session.summary_index.get(range)
}

/// Build the "not cached" response message.
pub fn not_cached_msg(start: usize, end: usize, target: usize) -> String {
    format!(
        "No cached summary for turns [{start},{end}) at \
         target_tokens={target}. Run a derivation that produces this \
         range, then call context_summarize again.",
    )
}
