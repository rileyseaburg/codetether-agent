//! MCP Server - Exposes CodeTether tools to MCP clients
//!
//! Runs as a stdio-based MCP server that can be connected to by:
//! - Claude Desktop
//! - Other MCP clients
//!
//! Exposed tools include:
//! - run_command: Execute shell commands
//! - read_file: Read file contents
//! - write_file: Write file contents
//! - search_files: Search for files
//! - swarm: Execute tasks with parallel sub-agents
//! - rlm: Analyze large content
//! - ralph: Autonomous PRD-driven execution

use anyhow::Result;
use codetether_bus_core::AgentBus;
use codetether_bus_http::BusBridge;
use codetether_mcp_core::server_catalog::{
    McpPromptHandler, McpResourceHandler, McpToolHandler, ServerCatalog,
};
use codetether_mcp_core::types::*;
use codetether_mcp_transport::{NullTransport, StdioTransport, Transport};
use serde_json::Value;
use std::{marker::PhantomData, sync::Arc};
mod host;
pub use host::Application;
/// MCP server using application-supplied registration and approval policy.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_mcp_server::Application>() {
/// let server = codetether_mcp_server::ApplicationServer::<H>::new_local();
/// # let _ = server;
/// # }
/// ```
pub struct ApplicationServer<H: Application> {
    inner: crate::ServerState<BusBridge, H::Registry>,
    _application: PhantomData<fn() -> H>,
}
mod attachment;
mod construction;
mod invocation;
mod prompt_catalog;
mod registry;
mod resource_catalog;
mod run;
mod runtime_host;
mod setup;
mod tool_catalog;
