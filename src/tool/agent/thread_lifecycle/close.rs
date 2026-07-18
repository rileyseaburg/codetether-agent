//! Idempotent explicit close of one durable child subtree.

use super::super::{collaboration_runtime::thread_status, persistence, residency};
use crate::tool::ToolResult;
use anyhow::Result;

/// Close a durable child and every currently resident descendant.
///
/// # Errors
///
/// Returns an error when durable identity lookup, persistence, or descendant
/// discovery fails.
pub(in crate::tool::agent) async fn run(target: &str, owner: Option<&str>) -> Result<ToolResult> {
    let Some(agent_id) = persistence::durable_id(owner, target).await? else {
        return Ok(ToolResult::error(format!("Agent {target} not found")));
    };
    let _transition = residency::acquire_transition(&agent_id).await;
    let closed = persistence::prepare_resume(owner, &agent_id).await?;
    let previous = thread_status::get(&agent_id);
    persistence::close(&closed.name, &closed.entry).await?;
    super::shutdown::one(&agent_id).await;
    super::shutdown::descendants(&agent_id).await?;
    Ok(super::result::closed(
        &closed.name,
        &closed.entry,
        &previous,
    ))
}
