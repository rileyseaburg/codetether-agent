//! One invocation attempt of a local sub-agent loop.

use super::super::run_agent_loop;
use super::{job::Job, retry::Outcome};
use crate::{provider::ToolDefinition, tool::ToolRegistry};
use std::sync::Arc;

pub(super) async fn execute(
    job: &Job,
    prompts: &(String, String),
    registry: &Arc<ToolRegistry>,
    tools: &[ToolDefinition],
) -> Outcome {
    run_agent_loop(
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
