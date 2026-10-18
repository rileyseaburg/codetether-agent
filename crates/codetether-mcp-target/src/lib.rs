//! Select the agent bus for an MCP server from CLI and project context.
//!
//! [resolve] preserves explicit bus, worker and workspace precedence before
//! project discovery and default-worker fallback.
mod project;
mod resolve;
pub use resolve::resolve;
mod default_worker;
mod explicit;
