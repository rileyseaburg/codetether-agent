//! Accumulate provider chunks into a buffered completion response.
use crate::*;
#[derive(Default)]
pub(crate) struct ToolAccumulator {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) arguments: String,
}

/// Collects streamed text, tool calls, reasoning, and completion usage.
///
/// # Examples
/// ```
/// let collector = codetether_codex_protocol::CompletionCollector::default();
/// assert!(collector.finish().is_err()); // A terminal Done chunk is required.
/// ```
#[derive(Default)]
pub struct CompletionCollector {
    pub(crate) text: String,
    pub(crate) reasoning_signature: Option<String>,
    pub(crate) tools: Vec<ToolAccumulator>,
    pub(crate) tool_indexes: HashMap<String, usize>,
    pub(crate) usage: Usage,
    pub(crate) saw_done: bool,
}

mod accept;
mod finish;
mod tools;
