//! Native coordinator and stage execution for the shared swarm executor.
use super::SwarmExecutor;
use crate::swarm::{DecompositionStrategy, SwarmResult};
/// Application services selected by the public swarm executor alias.
pub struct Native;
impl codetether_swarm_runtime::ExecutorHost for Native {
    type Agent = crate::agent::Agent;
    async fn execute(
        executor: &SwarmExecutor,
        task: &str,
        strategy: DecompositionStrategy,
    ) -> anyhow::Result<SwarmResult> {
        super::run::execute(executor, task, strategy).await
    }
}
