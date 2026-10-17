//! Nesting-depth guard for model-spawned sub-agents.

use codetether_config::Config;
use codetether_tool_core::ToolResult;

/// Check the configured nesting-depth ceiling for a proposed child.
/// # Arguments
/// * `name` — Proposed child display name, used in rejections.
/// * `depth` — Child depth derived from the current parent registry.
/// # Returns
/// Unit if the depth is within the configured ceiling.
/// # Errors
/// Returns a tool error if configuration cannot load or depth exceeds the limit.
/// # Examples
/// ```rust,no_run
/// # async fn example() {
/// let result = codetether_spawn_policy::validate_depth("reviewer", 1).await;
/// # }
/// ```
pub async fn validate(name: &str, depth: u8) -> Result<(), ToolResult> {
    let limit = Config::load()
        .await
        .map(|config| config.agents.max_depth())
        .map_err(|error| ToolResult::error(format!("cannot load agent depth limit: {error}")))?;
    if depth <= limit {
        return Ok(());
    }
    Err(ToolResult::error(format!(
        "cannot spawn @{} at depth {depth}; max depth is {limit}",
        name
    )))
}
