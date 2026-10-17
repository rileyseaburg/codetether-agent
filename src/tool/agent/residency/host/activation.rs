//! Native prepared-manifest and session-overlay bindings.
use super::super::super::{persistence, store};
use super::{ApplicationHost, Prepared};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_residency::{Host, ResumeConfig};
#[async_trait]
impl Host for ApplicationHost {
    type Prepared = Prepared;
    async fn prepare_resume(owner: Option<&str>, target: &str) -> Result<Prepared> {
        persistence::prepare_resume(owner, target)
            .await
            .map(Prepared)
    }
    fn entry_mut(prepared: &mut Prepared) -> &mut store::AgentEntry {
        &mut prepared.0.entry
    }
    async fn activate_resume(prepared: Prepared) -> Result<store::AgentEntry> {
        persistence::activate_resume(prepared.0).await
    }
    async fn overlay(entry: &mut store::AgentEntry, config: &ResumeConfig) -> Result<()> {
        super::super::overlay::apply(entry, config).await
    }
}
