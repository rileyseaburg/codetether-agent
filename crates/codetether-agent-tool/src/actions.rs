//! Small action helpers for the agent tool.
//!
//! This module contains lightweight dispatch helpers that do not warrant their
//! own full action modules.
//!
//! # Examples
//!
//! ```ignore
//! let result = unknown_action_result("oops");
//! ```

use crate::Host;
use anyhow::{Context, Result};
use codetether_agent_input::Params;
use codetether_tool_core::ToolResult;

/// Validates and executes the `kill` action for a named sub-agent.
///
/// # Examples
///
/// ```ignore
/// let result = execute_kill(&params)?;
/// ```
pub(super) async fn execute_kill<H: Host>(params: &Params) -> Result<ToolResult> {
    let name = params.name.as_deref().context("name required for kill")?;
    H::kill(name, params.parent_session_id.as_deref()).await
}

/// Interrupts a running turn while preserving the spawned agent.
pub(super) async fn execute_interrupt<H: Host>(params: &Params) -> Result<ToolResult> {
    let name = params
        .name
        .as_deref()
        .context("name required for interrupt")?;
    let marker_enabled = codetether_config::Config::load()
        .await?
        .agents
        .interrupt_message_enabled();
    H::interrupt(name, params.parent_session_id.as_deref(), marker_enabled).await
}

/// Formats the fallback error result for unsupported actions.
///
/// # Examples
///
/// ```ignore
/// let result = unknown_action_result("oops");
/// ```
pub(super) fn unknown_action_result(action: &str) -> ToolResult {
    ToolResult::error(format!(
        "Unknown action: {action}. Valid: spawn, message, list, status, interrupt, close, resume, kill"
    ))
}
