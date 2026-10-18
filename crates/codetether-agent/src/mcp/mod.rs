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

pub use codetether_bus_http as bus_bridge;
use codetether_mcp_server::payload as bus_payload;
use codetether_native_mcp_client as client;
mod server;
use codetether_mcp_core::types;
use codetether_native_mcp_client::transport;

pub use bus_bridge::BusBridge;
pub use client::{McpClient, McpRegistry};
pub use server::McpServer;
pub use transport::{NullTransport, SseTransport, StdioTransport, Transport};
pub use types::*;
