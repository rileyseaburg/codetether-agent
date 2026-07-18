//! Transactional activation of a closed durable child manifest.

use super::super::{execution_state, persistence, store::AgentEntry};
use super::ResumeConfig;
use anyhow::Result;

pub(super) async fn closed(
    target: &str,
    owner: Option<&str>,
    config: &ResumeConfig,
) -> Result<AgentEntry> {
    let mut closed = persistence::prepare_resume(owner, target).await?;
    super::overlay::apply(&mut closed.entry, config).await?;
    let agent_id = closed.entry.session.id.clone();
    execution_state::reopen(&agent_id);
    match persistence::activate_resume(closed).await {
        Ok(entry) => Ok(entry),
        Err(error) => {
            execution_state::close(&agent_id);
            Err(error)
        }
    }
}
