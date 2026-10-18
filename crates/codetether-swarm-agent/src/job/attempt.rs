//! One invocation attempt of a local sub-agent loop.

use super::JobHost;
use crate::run_agent_loop;
use codetether_provider_core::ToolDefinition;
use codetether_swarm_core::agent_retry::AgentOutcome as Outcome;
use codetether_swarm_local_job::Job;
use std::sync::Arc;

pub(super) async fn execute<H: JobHost>(
    job: &Job,
    prompts: &(String, String),
    registry: &Arc<H::Registry>,
    tools: &[ToolDefinition],
) -> Outcome {
    run_agent_loop::<H>(
        Arc::clone(&job.provider),
        &job.model,
        &prompts.0,
        &prompts.1,
        tools.to_vec(),
        Arc::clone(registry),
        job.max_steps,
        job.timeout_secs,
        job.events.clone(),
        job.id.clone(),
        job.bus.clone(),
        Some(job.workspace.clone()),
    )
    .await
}
