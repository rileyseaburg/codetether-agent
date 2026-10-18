//! Policy-bound MCP client connections and multi-server registration.
//!
//! [McpClient] owns a remote connection and its [McpRegistry]. Applications
//! implement [Application] to retain their identity and authorization rules.
//! Construct a client with a transport, or connect an authorized subprocess;
//! use its registry to discover and invoke tools across remote servers.
use anyhow::Result;
use codetether_mcp_core::types::*;
use codetether_mcp_transport::{ProcessTransport, Transport};
use serde_json::Value;
use std::{marker::PhantomData, sync::Arc};
mod application;
pub use application::Application;
mod call_tool;
mod construct;
mod initialize;
mod metadata;
mod operations;
mod registry;
mod registry_access;
mod registry_client;
mod subprocess;

/// Remote MCP client using the application's authorization rules.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_mcp_client::Application>() {
/// let transport = std::sync::Arc::new(codetether_mcp_transport::NullTransport::new());
/// let _client = codetether_mcp_client::McpClient::<H>::new(transport);
/// # }
/// ```
pub struct McpClient<H: Application> {
    inner: codetether_mcp_transport::client::RemoteClient,
    registry: Arc<McpRegistry<H>>,
    _application: PhantomData<fn() -> H>,
}

/// Registry whose connections use the same application policy as its clients.
/// # Examples
/// ```
/// # fn example<H: codetether_mcp_client::Application>() {
/// let _registry = codetether_mcp_client::McpRegistry::<H>::new();
/// # }
/// ```
pub struct McpRegistry<H: Application> {
    inner: codetether_mcp_core::registry::ServerRegistry<McpClient<H>>,
}

mod bridge_resources;
mod connection_client;
mod tool_client;
