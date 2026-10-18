//! State and effects required by single-tool lifecycle orchestration.
use crate::agent_loop::{ToolCall, tools::raw::RawResult};
use serde_json::Value;
use std::future::Future;

/// Bind a sub-agent's tool lifecycle to its current runtime state.
/// # Implementors
/// Native runtimes provide telemetry, policy/input preparation, raw execution,
/// full-output publication, and rendered feedback.
pub trait Host {
    /// Count the call and publish its start/trace observations in order.
    fn start(&mut self, call: &ToolCall);
    /// Parse and authorize the original arguments.
    fn prepare(&self, call: &ToolCall) -> impl Future<Output = Result<Value, String>> + Send;
    /// Execute prepared arguments under the agent deadline.
    fn raw(&self, name: &str, args: Value) -> impl Future<Output = RawResult> + Send;
    /// Publish completion details from the unrendered result.
    fn finished(&self, call: &ToolCall, result: &RawResult);
    /// Publish full tool output to agent observers.
    fn publish(&self, call: &ToolCall, result: &RawResult);
    /// Render the result, reporting a separate processing timeout.
    fn render(&self, name: &str, result: &RawResult)
    -> impl Future<Output = (String, bool)> + Send;
}
