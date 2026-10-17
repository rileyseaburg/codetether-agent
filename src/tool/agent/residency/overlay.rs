//! Application binding for shared live-parent session overlays.
use super::ResumeConfig;
use crate::tool::agent::store::AgentEntry;
pub(super) async fn apply(entry: &mut AgentEntry, config: &ResumeConfig) -> anyhow::Result<()> {
    codetether_agent_session::apply_overlay(entry, config).await
}
