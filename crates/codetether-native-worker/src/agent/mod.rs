//! Native standard/swarm selection using the shared worker outcome policy.
use crate::{WorkerTaskRuntime, swarm_policy::Session};
use codetether_a2a_worker_core::{execution_policy::agent::AgentOutcome, task_config::TaskContext};
use codetether_application_registry::{Application, ToolRegistry};
use codetether_bus_core::AgentBus;
use std::{marker::PhantomData, sync::Arc};
mod host;
struct Native<'a, S, T, W, E> {
    session: &'a mut S,
    runtime: &'a WorkerTaskRuntime,
    context: &'a TaskContext,
    output: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
    types: PhantomData<fn() -> (T, W, E)>,
}
/// Select and execute the task's standard or swarm agent.
/// # Arguments
/// Receives the prepared session, worker services, task context, agent selector
/// and output observer used by the existing shared selection policy.
/// # Returns
/// The selected agent's mapped outcome.
/// # Errors
/// Propagates standard-session or swarm execution errors.
pub async fn execute_task_agent<S, T, W, E>(
    session: &mut S,
    runtime: &WorkerTaskRuntime,
    context: &TaskContext,
    agent_type: &str,
    output_callback: Arc<dyn Fn(String) + Send + Sync + 'static>,
) -> anyhow::Result<AgentOutcome>
where
    S: Session,
    T: Application,
    W: codetether_worker_tools::Host<Registry = ToolRegistry<T>>,
    E: codetether_swarm_runtime::inline::Executor<Bus = AgentBus>,
{
    codetether_a2a_worker_core::execution_policy::agent::execute(
        &mut Native::<S, T, W, E> {
            session,
            runtime,
            context,
            output: Some(output_callback),
            types: PhantomData,
        },
        agent_type,
    )
    .await
}
