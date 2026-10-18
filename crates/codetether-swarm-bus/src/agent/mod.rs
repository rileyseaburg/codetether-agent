//! Per-agent thinking and full tool-output publication.
//!
//! These functions publish under the subtask's bus identity, preserving the
//! shared bounded payload policy. Missing buses make publication a no-op.
mod thinking;
mod tool;
pub use thinking::publish_thinking;
pub use tool::publish_tool_output;
