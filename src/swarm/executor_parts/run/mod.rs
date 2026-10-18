//! Bind the shared swarm run lifecycle to native executor services.
mod host;
use super::SwarmExecutor;
use crate::swarm::{DecompositionStrategy, SwarmResult};
pub(super) async fn execute(
    executor: &SwarmExecutor,
    task: &str,
    strategy: DecompositionStrategy,
) -> anyhow::Result<SwarmResult> {
    codetether_swarm_runtime::execute(&host::Native(executor), task, strategy).await
}
