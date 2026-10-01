//! First-party RustyRoad access through its published MCP executable.
//!
//! Each invocation owns an isolated, short-lived subprocess. Install the
//! backend with `cargo install rustyroad --locked --bin rustyroad-mcp`.

mod command;
mod environment;
mod initialize;
mod invocation;
mod params;
mod process;
mod response;
mod rpc;
mod run;
mod schema;
mod session;
mod tool_impl;

#[cfg(test)]
mod tests;

/// Built-in adapter for the `rustyroad-mcp` executable on PATH.
pub(crate) struct RustyRoadTool;
