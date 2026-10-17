//! Worker tool-call approval and session-effect coordination.
//!
//! [execute_tool_call] preserves invocation events, approval decisions,
//! argument preparation and result ordering through [ToolCallHost].
mod host;
mod run;
pub use host::ToolCallHost;
pub use run::execute_tool_call;
