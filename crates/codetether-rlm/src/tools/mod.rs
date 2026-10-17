//! RLM tool schemas, bounded output, and in-memory operation dispatch.
//!
//! [rlm_tool_definitions] describes the eight operations. [dispatch_tool_call]
//! executes recognized operations against a [crate::repl::RlmRepl], returning
//! [RlmToolResult]. Provider sub-queries are returned as a sentinel for the host.
mod definitions;
mod dispatch;
mod output;
mod result;
pub use definitions::rlm_tool_definitions;
pub use dispatch::dispatch_tool_call;
pub use result::RlmToolResult;
#[cfg(test)]
mod tests;
