//! Root-only resume for one durable child identity.

use super::super::{collaboration_runtime::thread_status, residency, store};
use crate::tool::ToolResult;
use anyhow::{Context, Result};

/// Resume only the selected child while retaining lazy descendant identities.
///
/// # Errors
///
/// Returns an error when the session cannot be restored or registered.
pub(in crate::tool::agent) async fn run(
    target: &str,
    owner: Option<&str>,
    config: residency::ResumeConfig,
) -> Result<ToolResult> {
    match residency::open(target, owner, config).await? {
        residency::EnsureOpen::Ready { agent_id, .. } => {
            let entry = store::get(&agent_id).context("resident child disappeared")?;
            let status = thread_status::get(&agent_id);
            Ok(super::result::resumed(&entry.name, &entry, &status))
        }
        residency::EnsureOpen::Missing => {
            Ok(ToolResult::error(format!("Agent {target} not found")))
        }
        residency::EnsureOpen::Rejected(result) => Ok(result),
    }
}
