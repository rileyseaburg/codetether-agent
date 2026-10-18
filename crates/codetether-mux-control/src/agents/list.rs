use anyhow::Result;
use serde_json::Value;

/// Project authenticated mux sessions into agent-tool roster entries.
///
/// # Returns
/// JSON entries with stable route IDs, display names and runtime status.
/// # Errors
/// Returns an error when registered mux sessions cannot be listed.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let entries = codetether_mux_control::agent_sessions().await?;
/// assert!(entries.iter().all(|entry| entry["kind"] == "mux_session"));
/// # Ok(()) }
/// ```
pub async fn agent_sessions() -> Result<Vec<Value>> {
    Ok(crate::list_sessions()
        .await?
        .into_iter()
        .map(super::project::entry)
        .collect())
}
