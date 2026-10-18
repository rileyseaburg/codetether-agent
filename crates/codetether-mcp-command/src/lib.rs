//! MCP slash-command parsing, execution and response construction.
//!
//! execute dispatches through a caller-owned Connections registry and returns
//! CommandOutput. Callers apply status and optional message to their own UI.
mod call;
mod call_result;
mod connect;
mod execute;
mod servers;
mod tools;
pub use execute::execute;
/// Status and optional transcript notice produced by an MCP command.
/// # Examples
/// ```
/// let output = codetether_mcp_command::CommandOutput::default();
/// assert!(output.message.is_none());
/// ```
#[derive(Default)]
pub struct CommandOutput {
    /// Status-line text to display.
    pub status: String,
    /// System message to append when present.
    pub message: Option<String>,
}

/// Frontend command execution and result presentation.
pub mod interactive;
