//! Native entry to shared worker agent selection and result mapping.
mod host;
use super::{TaskContext, WorkerTaskRuntime};
use crate::session::Session;
use std::sync::Arc;
pub(super) async fn execute_task_agent(
    session: &mut Session,
    runtime: &WorkerTaskRuntime,
    context: &TaskContext,
    agent_type: &str,
    output_callback: Arc<dyn Fn(String) + Send + Sync + 'static>,
) -> anyhow::Result<codetether_a2a_worker_core::execution_policy::agent::AgentOutcome> {
    codetether_a2a_worker_core::execution_policy::agent::execute(
        &mut host::Native {
            session,
            runtime,
            context,
            output: Some(output_callback),
        },
        agent_type,
    )
    .await
}
