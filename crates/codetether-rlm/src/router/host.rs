//! Host-provided trait for RLM tool dispatch.
//!
//! The router needs context inspection, structured tool dispatch, and tool
//! definitions. The shared [`super::ReplHost`] supplies the standard in-memory
//! implementation; embedding applications can provide alternatives.

use crate::traits::ToolDefinition;

/// Result of dispatching a single RLM tool call.
#[derive(Debug, Clone)]
pub enum HostToolResult {
    /// Normal output, fed back to the LLM.
    Output(String),
    /// Final answer — terminates the RLM loop.
    Final(String),
}

/// Narrow interface the router needs from the host environment.
///
/// # Implementors
/// [`super::ReplHost`] uses the shared REPL and tool dispatcher.
pub trait RouterHost: Send {
    /// Return the `rlm_head/tail/grep/…` tool definitions.
    fn tool_definitions(&self) -> Vec<ToolDefinition>;

    /// Dispatch one tool call. Returns `None` for unknown tools.
    fn dispatch(&mut self, name: &str, arguments: &str) -> Option<HostToolResult>;
}
