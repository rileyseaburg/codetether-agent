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

use super::handlers;
use super::params::Params;
use crate::tool::ToolResult;
use anyhow::{Context, Result};

/// Validates and executes the `kill` action for a named sub-agent.
///
/// # Examples
///
/// ```ignore
/// let result = execute_kill(&params)?;
/// ```
pub(super) fn execute_kill(params: &Params) -> Result<ToolResult> {
    let name = params.name.as_deref().context("name required for kill")?;
    Ok(handlers::handle_kill(name))
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
        "Unknown action: {action}. Valid: spawn, message, list, kill"
    ))
}
