//! Lifecycle of one spawned local sub-agent.

use super::super::workspace_registry;
use super::{finish_events, job::Job, prompts, result, retry, start_events};
use crate::swarm::SubTaskResult;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub(super) async fn execute(job: Job) -> (String, anyhow::Result<SubTaskResult>) {
    let id = job.id.clone();
    if job.stagger_ms > 0 {
        tokio::time::sleep(Duration::from_millis(job.stagger_ms)).await;
    }
    let _permit = match job.semaphore.acquire().await {
        Ok(permit) => permit,
        Err(_) => return (id, Err(anyhow::anyhow!("Swarm execution cancelled"))),
    };
    let started = Instant::now();
    let prompts = prompts::build(&job);
    start_events::emit(&job);
    let capability = workspace_registry::Capability::from_flags(job.read_only, job.verification);
    let (registry, tools) = workspace_registry::for_agent(
        Arc::clone(&job.provider),
        job.model.clone(),
        &job.workspace,
        capability,
        Arc::clone(&job.result_store),
        job.id.clone(),
    );
    let (outcome, retries) = retry::execute(&job, &prompts, registry, tools).await;
    let (result, status) = result::build(&job, outcome, retries, started.elapsed());
    finish_events::emit(&job, &result, status);
    (id, Ok(result))
}
