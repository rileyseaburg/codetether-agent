//! MCP (Model Context Protocol) implementation
//!
//! MCP is a standardized protocol for connecting AI models to external tools
//! and data sources. This implementation supports:
//! - JSON-RPC 2.0 messaging over stdio/SSE
//! - Tool definitions and invocation
//! - Resource exposure and sampling
//! - Prompt templates
//!
//! # Server Mode
//! Run as an MCP server to expose CodeTether tools to Claude Desktop or other clients:
//! ```bash
//! codetether mcp serve
//! ```
//!
//! # Client Mode
//! Connect to external MCP servers to use their tools:
//! ```bash
//! codetether mcp connect "npx -y @modelcontextprotocol/server-filesystem /path"
//! ```

mod approval;
pub use codetether_bus_http as bus_bridge;
use codetether_mcp_server::payload as bus_payload;
mod client;
mod fallback_grep;
mod fallback_run_command;
mod fallback_search;
mod fallback_search_files;
mod policy_host;
mod server;
mod subprocess_policy;
mod tool_policy;
mod tool_result;
mod transport;
use codetether_mcp_core::types;

pub use bus_bridge::BusBridge;
pub use client::{McpClient, McpRegistry};
pub use server::McpServer;
pub use transport::{NullTransport, SseTransport, StdioTransport, Transport};
pub use types::*;

#[cfg(test)]
mod image_server_tests;
