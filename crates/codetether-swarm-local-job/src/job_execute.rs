//! Lifecycle of one spawned local sub-agent.

use super::{finish_events, job::Job, prompts, result, retry, start_events};
use codetether_swarm_core::SubTaskResult;
use std::time::{Duration, Instant};

pub(super) async fn execute<H: crate::Host>(job: Job) -> (String, anyhow::Result<SubTaskResult>) {
    let id = job.id.clone();
    if job.stagger_ms > 0 {
        tokio::time::sleep(Duration::from_millis(job.stagger_ms)).await;
    }
    let _permit = match job.semaphore.acquire().await {
        Ok(permit) => permit,
        Err(_) => return (id, Err(anyhow::anyhow!("Swarm execution cancelled"))),
    };
    let started = Instant::now();
    let prompts = prompts::build::<H>(&job);
    start_events::emit(&job);
    let (registry, tools) = H::registry(&job);
    let (outcome, retries) = retry::execute::<H>(&job, &prompts, registry, tools).await;
    let (result, status) = result::build(&job, outcome, retries, started.elapsed());
    finish_events::emit(&job, &result, status);
    (id, Ok(result))
}
