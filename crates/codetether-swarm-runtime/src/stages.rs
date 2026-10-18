//! Application entry point for shared dependency-stage traversal.
use super::state::Run;
use anyhow::Result;
pub(super) async fn execute(run: &mut Run<'_, impl crate::Host>) -> Result<()> {
    codetether_swarm_core::stage_runtime::execute_stages(run).await
}
