//! Bounded tracking of in-flight tool calls observed on the agent bus.
//!
//! Feed ToolCallTracker with bus requests and responses, then query active
//! calls, declared budgets or stalls for watchdog and agent-status views.
mod tool_calls;
mod tool_calls_query;
pub use tool_calls::{OpenToolCall, ToolCallTracker};
#[cfg(test)]
mod tool_calls_limit_tests;
#[cfg(test)]
mod tool_calls_tests;
