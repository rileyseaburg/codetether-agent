//! Shared server registry, capability metadata and tool routing.
//!
//! [ServerRegistry] retains caller-supplied clients; [RegistryClient] keeps
//! connection construction and authorization in the application.
use crate::types::*;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
mod contract;
pub use contract::RegistryClient;
mod all_tools;
mod capabilities;
mod construct;
mod disconnect;
mod dispatch;
mod find_tool;
mod lookup;
mod refresh;
mod register;

/// MCP server registry holding caller-defined client connections.
///
/// # Examples
/// ```
/// fn empty<C: codetether_mcp_core::registry::RegistryClient>() -> codetether_mcp_core::registry::ServerRegistry<C> {
///     codetether_mcp_core::registry::ServerRegistry::new()
/// }
/// ```
///
/// This registry allows managing connections to multiple external MCP servers,
/// enabling CodeTether to use tools from various sources like filesystem servers,
/// database servers, and custom tool servers.
pub struct ServerRegistry<C> {
    clients: RwLock<HashMap<String, Arc<C>>>,
    /// Track server capabilities for quick lookup without querying each client
    server_capabilities: RwLock<HashMap<String, ServerCapabilities>>,
    /// Track all discovered tools across servers for efficient discovery
    tool_index: RwLock<HashMap<String, String>>, // tool_name -> server_name
}
