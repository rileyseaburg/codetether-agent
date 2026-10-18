//! Native coordinator and stage execution for the shared swarm executor.
use codetether_application_registry::Application;
use codetether_swarm_core::{DecompositionStrategy, SwarmResult};
use codetether_swarm_runtime::SwarmExecutor;
use std::marker::PhantomData;
/// Application services selected by the public swarm executor alias.
/// Native stage services using tool application T and coordinator agent A.
/// # Examples
/// Select ExecutorHost<T, A> as the host of the shared SwarmExecutor.
pub struct ExecutorHost<T, A>(PhantomData<fn() -> (T, A)>);
impl<T: Application, A: Send + 'static> codetether_swarm_runtime::ExecutorHost
    for ExecutorHost<T, A>
{
    type Agent = A;
    async fn execute(
        executor: &SwarmExecutor<Self>,
        task: &str,
        strategy: DecompositionStrategy,
    ) -> anyhow::Result<SwarmResult> {
        codetether_swarm_runtime::execute(executor, task, strategy).await
    }
}

impl<T: Application, A: Send + 'static> codetether_swarm_runtime::LocalStageHost
    for ExecutorHost<T, A>
{
    type Providers = codetether_native_providers::ProviderRegistry;
    fn list(providers: &Self::Providers) -> Vec<&str> {
        providers.list()
    }
    fn get(
        providers: &Self::Providers,
        name: &str,
    ) -> Option<std::sync::Arc<dyn codetether_provider_core::Provider>> {
        providers.get(name)
    }
    fn run_job(
        job: codetether_swarm_local_job::Job,
    ) -> impl std::future::Future<
        Output = (String, anyhow::Result<codetether_swarm_core::SubTaskResult>),
    > + Send
    + 'static {
        codetether_swarm_agent::job::execute::<crate::AgentHost<T>>(job)
    }
}

impl<T: Application, A: Send + 'static> codetether_swarm_runtime::ExecutionHost
    for ExecutorHost<T, A>
{
    type Planning = crate::Providers;
}
