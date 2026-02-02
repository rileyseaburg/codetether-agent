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

mod types;
mod server;
mod client;
mod transport;

pub use types::*;
pub use server::McpServer;
pub use client::McpClient;
pub use transport::{Transport, StdioTransport, SseTransport};
