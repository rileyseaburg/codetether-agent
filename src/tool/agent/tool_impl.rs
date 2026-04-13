//! Top-level tool implementation for sub-agent operations.
//!
//! This module holds the `Tool` trait implementation and dispatches parsed
//! actions to focused submodules.
//!
//! # Examples
//!
//! ```ignore
//! let tool = AgentTool::new();
//! ```

use super::actions::execute_kill;
use super::{handlers, message, spawn};
use crate::tool::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::Value;

/// Tool entrypoint for spawning and managing sub-agents.
///
/// The tool supports `spawn`, `message`, `list`, and `kill` actions and
/// delegates the implementation to narrower modules.
///
/// # Examples
///
/// ```ignore
/// let tool = AgentTool::new();
/// assert_eq!(tool.name(), "Sub-Agent");
/// ```
pub struct AgentTool;

impl AgentTool {
    /// Creates the sub-agent management tool.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let tool = AgentTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}

impl Default for AgentTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for AgentTool {
    fn id(&self) -> &str {
        "agent"
    }

    fn name(&self) -> &str {
        "Sub-Agent"
    }

    fn description(&self) -> &str {
        "Spawn and communicate with specialized sub-agents. Actions: spawn, message, list, kill. Spawned agents must use a free/subscription-eligible model."
    }

    fn parameters(&self) -> Value {
        super::tool_schema::agent_tool_parameters()
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let parsed: super::params::Params =
            serde_json::from_value(params).context("Invalid params")?;
        match parsed.action.as_str() {
            "spawn" => spawn::handle_spawn(&parsed).await,
            "message" => message::handle_message(&parsed).await,
            "list" => Ok(handlers::handle_list()),
            "kill" => execute_kill(&parsed),
            _ => Ok(super::actions::unknown_action_result(&parsed.action)),
        }
    }
}
