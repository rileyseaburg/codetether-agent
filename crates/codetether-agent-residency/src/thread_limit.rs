//! Resident and pending sub-agent thread limit configuration.

use codetether_config::Config;
use codetether_tool_core::ToolResult;

/// Resolve the configured maximum number of resident or starting children.
/// # Arguments
/// This function takes no arguments.
/// # Returns
/// The configured resident-plus-pending capacity.
/// # Errors
/// Returns a tool error when configuration cannot be loaded.
/// # Examples
/// ```rust,no_run
/// # async fn example() {
/// let capacity = codetether_agent_residency::limit().await;
/// assert!(capacity.is_ok());
/// # }
/// ```
pub async fn limit() -> Result<usize, ToolResult> {
    Config::load()
        .await
        .map(|config| config.agents.max_threads())
        .map_err(|error| ToolResult::error(format!("cannot load agent thread limit: {error}")))
}

/// Build the structured rejection returned when residency is full.
/// # Arguments
/// * `usage` — Current resident-plus-pending count.
/// * `limit` — Configured maximum.
/// # Returns
/// A failed tool result explaining the capacity limit.
/// # Examples
/// ```rust
/// assert!(!codetether_agent_residency::rejected(6, 6).success);
/// ```
pub fn rejected(usage: usize, limit: usize) -> ToolResult {
    ToolResult::error(format!(
        "cannot open agent: {usage} threads are resident or starting; max is {limit}"
    ))
}
