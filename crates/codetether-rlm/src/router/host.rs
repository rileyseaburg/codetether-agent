//! Host-provided trait for RLM tool dispatch.
//!
//! The router needs three host-defined capabilities that live in
//! `codetether-agent`: the REPL context, structured tool dispatch,
//! and tool-definition generation. This trait exposes them without
//! coupling the crate to concrete types.

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
/// Implemented by `codetether-agent` with `RlmRepl` and the
/// `dispatch_tool_call` function from `src/rlm/tools.rs`.
pub trait RouterHost: Send + Sync {
    /// Return the `rlm_head/tail/grep/…` tool definitions.
    fn tool_definitions(&self) -> Vec<ToolDefinition>;

    /// Dispatch one tool call. Returns `None` for unknown tools.
    fn dispatch(&mut self, name: &str, arguments: &str) -> Option<HostToolResult>;
}
