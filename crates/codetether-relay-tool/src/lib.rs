//! Relay coordination tool, private runtime registry and action validation.
//!
//! [RelayAutoChatTool] initializes relays and publishes protocol handoffs.
//! It keeps participant lists scoped to relay IDs on a dedicated private bus.
use anyhow::Result;
use async_trait::async_trait;
use codetether_bus_core::{AgentBus, relay::ProtocolRelayRuntime};
use codetether_tool_core::{Tool, ToolResult};
use parking_lot::RwLock;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::OnceCell;
use uuid::Uuid;
mod action;
mod complete_relay;
mod construct;
mod delegate_params;
mod delegate_task;
mod execute;
mod get_or_create_runtime;
mod get_status;
mod handoff_context;
mod handoff_message;
mod handoff_params;
mod init_relay;
mod list_agents;
mod params;
mod remember_participant;
mod schema;
mod state;
mod storage;
#[cfg(test)]
mod tests;
mod tool;
use state::RelayRuntimeState;
use storage::{RELAY_STORE, get_agent_bus};
/// Relay initialization, handoff, status and completion tool.
///
/// # Examples
/// ```rust
/// let _tool = codetether_relay_tool::RelayAutoChatTool::new();
/// ```
pub struct RelayAutoChatTool;
