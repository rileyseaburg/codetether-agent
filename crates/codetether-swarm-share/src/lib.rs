//! Tool adapter for publishing and querying intermediate swarm results.
//!
//! [SwarmShareTool] binds a shared [ResultStore] to the publishing agent.
//! Construct it with a store and identity, then invoke the shared Tool contract.
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_swarm_core::result_store::ResultStore;
use codetether_tool_core::{Tool, ToolResult};
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;
mod construct;
mod execute;
mod get;
mod list;
mod params;
mod publish;
mod query_prefix;
mod query_tags;
mod schema;
mod tool;
use params::Params;
/// Shares intermediate results through a swarm-owned result store.
///
/// # Examples
/// ```rust
/// let _tool = codetether_swarm_share::SwarmShareTool::with_defaults();
/// ```

pub struct SwarmShareTool {
    store: Arc<ResultStore>,
    /// The subtask ID of the agent using this tool
    producer_id: String,
}
