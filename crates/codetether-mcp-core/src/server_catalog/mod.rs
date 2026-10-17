//! Registered MCP handlers and metadata, independent of application bus wiring.
//!
//! Registration and protocol listing share one store. Callers retain tool
//! authorization and live event publication.
use crate::types::*;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
mod handlers;
pub use handlers::{McpPromptHandler, McpResourceHandler, McpToolHandler};
mod access;
mod direct;
mod list_resources;
mod list_tools;
mod metadata;
mod prompts;
mod read_resource;
mod register_resource;
mod register_tool;
/// Synchronized handler and metadata maps for one MCP server.
///
/// # Examples
/// ```
/// let catalog = codetether_mcp_core::server_catalog::ServerCatalog::new();
/// ```
pub struct ServerCatalog {
    tools: RwLock<HashMap<String, McpToolHandler>>,
    resources: RwLock<HashMap<String, McpResourceHandler>>,
    prompts: RwLock<HashMap<String, McpPromptHandler>>,
    metadata: RwLock<HashMap<String, ToolMetadata>>,
    resource_metadata: RwLock<HashMap<String, ResourceMetadata>>,
}
impl ServerCatalog {
    /// Create empty handler and metadata maps.
    pub fn new() -> Self {
        Self {
            tools: RwLock::new(HashMap::new()),
            resources: RwLock::new(HashMap::new()),
            prompts: RwLock::new(HashMap::new()),
            metadata: RwLock::new(HashMap::new()),
            resource_metadata: RwLock::new(HashMap::new()),
        }
    }
}
