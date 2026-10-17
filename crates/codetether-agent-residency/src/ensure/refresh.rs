//! One-time live configuration refresh for disk-restored children.

use crate::{Host, ResumeConfig};
use codetether_agent_registry::AgentEntry;
use codetether_collaboration_state::residency::stale;

pub(super) async fn restored<H: Host>(
    agent_id: &str,
    entry: &mut AgentEntry<H::Session>,
    config: &ResumeConfig,
) -> anyhow::Result<()> {
    if config.is_empty() || !stale::take(agent_id) {
        return Ok(());
    }
    if let Err(error) = H::overlay(entry, config).await {
        stale::mark(agent_id);
        return Err(error);
    }
    H::insert(entry.clone());
    Ok(())
}
