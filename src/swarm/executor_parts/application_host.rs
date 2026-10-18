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
        codetether_swarm_runtime::execute(executor, task, strategy).await
    }
}

impl codetether_swarm_runtime::LocalStageHost for Native {
    type Providers = crate::provider::ProviderRegistry;
    fn list(providers: &Self::Providers) -> Vec<&str> {
        providers.list()
    }
    fn get(
        providers: &Self::Providers,
        name: &str,
    ) -> Option<std::sync::Arc<dyn crate::provider::Provider>> {
        providers.get(name)
    }
    fn run_job(
        job: codetether_swarm_local_job::Job,
    ) -> impl std::future::Future<Output = (String, anyhow::Result<crate::swarm::SubTaskResult>)>
    + Send
    + 'static {
        super::local_stage::job_run::execute(job)
    }
}

impl codetether_swarm_runtime::ExecutionHost for Native {
    type Planning = crate::swarm::orchestrator::NativeProvider;
}
