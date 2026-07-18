//! Application of live parent settings to one restored child session.

use super::ResumeConfig;
use crate::tool::agent::store::AgentEntry;
use anyhow::Result;

#[path = "overlay/metadata.rs"]
mod metadata;
#[path = "overlay/prompt.rs"]
mod prompt;
#[path = "overlay/workspace.rs"]
mod workspace;

pub(super) async fn apply(entry: &mut AgentEntry, config: &ResumeConfig) -> Result<()> {
    let metadata_changed = metadata::apply(entry, config);
    let workspace_changed = workspace::apply(entry, config);
    if metadata_changed || workspace_changed {
        entry.session.updated_at = chrono::Utc::now();
        entry.session.save().await?;
    }
    Ok(())
}
