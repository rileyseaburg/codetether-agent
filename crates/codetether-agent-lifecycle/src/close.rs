//! Idempotent explicit close of one durable child subtree.

use crate::Host;
use anyhow::Result;
use codetether_collaboration_state::{residency, thread_status};
use codetether_tool_core::ToolResult;

/// Close a durable child and every currently resident descendant.
///
/// # Arguments
/// * `target` — Durable ID or owner-scoped display name.
/// * `owner` — Optional owner scope.
/// # Returns
/// A structured close result, or a not-found tool result.
/// # Examples
/// ```rust
/// use codetether_agent_lifecycle::{close, Host};
/// async fn apply<H: Host>() -> anyhow::Result<()> {
///     let result = close::<H>("child", Some("owner")).await?;
///     assert!(result.success);
///     Ok(())
/// }
/// ```
/// # Errors
///
/// Returns an error when durable identity lookup, persistence, or descendant
/// discovery fails.
pub async fn run<H: Host>(target: &str, owner: Option<&str>) -> Result<ToolResult> {
    let Some(agent_id) = H::durable_id(owner, target).await? else {
        return Ok(ToolResult::error(format!("Agent {target} not found")));
    };
    let _transition = residency::acquire(&agent_id).await;
    let (name, entry) = H::prepare_resume(owner, &agent_id).await?;
    let previous = thread_status::get(&agent_id);
    H::close(&name, &entry).await?;
    super::shutdown::one::<H>(&agent_id).await;
    super::shutdown::descendants::<H>(&agent_id).await?;
    Ok(super::result::closed(&name, &entry, &previous))
}
