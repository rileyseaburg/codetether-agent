//! Transactional activation of a closed durable child manifest.

use crate::Host;
use crate::ResumeConfig;
use anyhow::Result;
use codetether_agent_registry::AgentEntry;
use codetether_collaboration_state::execution as execution_state;

pub(super) async fn closed<H: Host>(
    target: &str,
    owner: Option<&str>,
    config: &ResumeConfig,
) -> Result<AgentEntry<H::Session>> {
    let mut closed = H::prepare_resume(owner, target).await?;
    H::overlay(H::entry_mut(&mut closed), config).await?;
    let agent_id = H::entry_mut(&mut closed).id().to_owned();
    execution_state::reopen(&agent_id);
    match H::activate_resume(closed).await {
        Ok(entry) => Ok(entry),
        Err(error) => {
            execution_state::close(&agent_id);
            Err(error)
        }
    }
}
