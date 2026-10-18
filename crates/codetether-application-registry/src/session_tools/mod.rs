//! Session-specific additions and restrictions for an application tool registry.
//!
//! [`register_mcp`] installs the configured authenticated MCP connection.
//! [`remove_prior_context`] removes tools under the shared prior-context policy.
mod mcp;
mod prior_context;
pub use mcp::register_mcp;
pub use prior_context::remove_prior_context;
