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

use super::bus_bridge::BusBridge;
use super::transport::{StdioTransport, Transport};
use super::types::*;
use crate::bus::{AgentBus, BusMessage};
use crate::tool::ToolRegistry;
use anyhow::Result;
use codetether_mcp_core::server_catalog::{
    McpPromptHandler, McpResourceHandler, McpToolHandler, ServerCatalog,
};
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::RwLock;

/// MCP Server implementation
pub struct McpServer {
    transport: Arc<dyn Transport>,
    catalog: ServerCatalog,
    initialized: RwLock<bool>,
    server_info: ServerInfo,
    /// Optional bus bridge for live event monitoring
    bus: Option<Arc<BusBridge>>,
    /// Optional local agent bus for publishing tool calls to S3 sink
    agent_bus: Option<Arc<AgentBus>>,
    /// Optional tool registry bridging all CodeTether tools to MCP
    tool_registry: Option<Arc<ToolRegistry>>,
}

mod attachment;
mod bus;
mod construction;
mod fallback;
mod invocation;
mod prompt_catalog;
mod registry;
mod resource_catalog;
mod run;
mod runtime_host;
mod setup;
mod tool_catalog;
