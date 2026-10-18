//! Shared batch execution using the application's registry and runtime policy.
use super::ToolRegistry;
/// Batch tool bound to the application's registered tools.
///
/// # Examples
/// ```rust
/// let _tool = codetether_agent::tool::batch::BatchTool::new();
/// ```
pub type BatchTool = codetether_batch_tool::BatchTool<ToolRegistry>;
#[cfg(test)]
#[path = "batch_policy.rs"]
mod batch_policy;
