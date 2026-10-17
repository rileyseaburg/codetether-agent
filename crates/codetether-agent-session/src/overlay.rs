//! Application of live parent settings to one restored child session.

use crate::ChildSession;
use anyhow::Result;
use codetether_agent_lifecycle::ResumeConfig;
use codetether_agent_registry::AgentEntry;

#[path = "overlay/metadata.rs"]
mod metadata;
#[path = "overlay/prompt.rs"]
mod prompt;
#[path = "overlay/workspace.rs"]
mod workspace;

/// Apply parent overrides while preserving a task-owned pinned workspace.
/// # Arguments
/// * `entry` — Existing live child entry to refresh in place.
/// * `config` — Optional current-parent overrides.
/// # Returns
/// Unit after saving only when metadata or workspace actually changed.
/// # Errors
/// Propagates the native session save error.
/// # Examples
/// See [crate::ChildSession] for application operations used by this function.
pub async fn apply<S: ChildSession>(
    entry: &mut AgentEntry<S>,
    config: &ResumeConfig,
) -> Result<()> {
    let metadata_changed = metadata::apply(entry, config);
    let workspace_changed = workspace::apply(entry, config);
    if metadata_changed || workspace_changed {
        entry.session.save_updated().await?;
    }
    Ok(())
}
