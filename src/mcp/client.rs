//! MCP Client - Connect to external MCP servers
//!
//! Allows CodeTether to use tools from other MCP servers:
//! - Filesystem servers
//! - Database servers
//! - API integration servers
//! - Custom tool servers

use super::transport::{ProcessTransport, Transport};
use super::types::*;
use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;

#[path = "client_call_tool.rs"]
mod call_tool;
#[path = "client_subprocess.rs"]
mod subprocess;

/// MCP Client for connecting to external servers
pub struct McpClient {
    inner: codetether_mcp_transport::client::RemoteClient,
    /// Registry for managing multiple MCP server connections and capability tracking
    registry: Arc<McpRegistry>,
}
mod construct;
mod initialize;
mod metadata;
mod operations;
mod registry_access;
/// MCP registry with application-authorized connection setup.
pub struct McpRegistry {
    inner: codetether_mcp_core::registry::ServerRegistry<McpClient>,
}

#[path = "client_registry.rs"]
mod registry;
#[path = "client_registry_client.rs"]
mod registry_client;
