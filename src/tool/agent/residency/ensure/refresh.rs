//! One-time live configuration refresh for disk-restored children.

use super::super::ResumeConfig;
use crate::tool::agent::store::{self, AgentEntry};

pub(super) async fn restored(
    agent_id: &str,
    entry: &mut AgentEntry,
    config: &ResumeConfig,
) -> anyhow::Result<()> {
    if config.is_empty() || !super::super::stale::take(agent_id) {
        return Ok(());
    }
    if let Err(error) = super::super::overlay::apply(entry, config).await {
        super::super::stale::mark(agent_id);
        return Err(error);
    }
    store::insert(entry.clone());
    Ok(())
}
