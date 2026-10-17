//! Root-only resume for one durable child identity.

use crate::{Host, ResumeConfig};
use anyhow::{Context, Result};
use codetether_collaboration_state::thread_status;
use codetether_collaboration_tools::EnsureOpen;
use codetether_tool_core::ToolResult;

/// Resume only the selected child while retaining lazy descendant identities.
///
/// # Arguments
/// * `target` — Durable ID or owner-scoped display name.
/// * `owner` — Optional owner scope.
/// * `config` — Current parent's activation overrides.
/// # Returns
/// A structured resume result or the activation rejection/not-found result.
/// # Examples
/// ```rust
/// use codetether_agent_lifecycle::{resume, Host};
/// async fn apply<H: Host>() -> anyhow::Result<()> {
///     let result = resume::<H>("child", Some("owner"), Default::default()).await?;
///     assert!(result.success);
///     Ok(())
/// }
/// ```
/// # Errors
///
/// Returns an error when the session cannot be restored or registered.
pub async fn run<H: Host>(
    target: &str,
    owner: Option<&str>,
    config: ResumeConfig,
) -> Result<ToolResult> {
    match H::open(target, owner, config).await? {
        EnsureOpen::Ready { agent_id, .. } => {
            let entry = H::get(&agent_id).context("resident child disappeared")?;
            let status = thread_status::get(&agent_id);
            Ok(super::result::resumed(&entry.name, &entry, &status))
        }
        EnsureOpen::Missing => Ok(ToolResult::error(format!("Agent {target} not found"))),
        EnsureOpen::Rejected(result) => Ok(result),
    }
}
