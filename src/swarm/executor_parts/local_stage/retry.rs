//! Deadline-aware retry loop for a local sub-agent.

use super::super::{AgentLoopExit, backoff};
use super::job::Job;
use crate::provider::ToolDefinition;
use crate::tool::ToolRegistry;
use anyhow::Result;
use std::sync::Arc;

pub(super) type Outcome = Result<(String, usize, usize, AgentLoopExit)>;

pub(super) async fn execute(
    job: &Job,
    prompts: &(String, String),
    registry: Arc<ToolRegistry>,
    tools: Vec<ToolDefinition>,
) -> (Outcome, u32) {
    let mut attempt = 0;
    loop {
        let outcome = super::retry_attempt::execute(job, prompts, &registry, &tools).await;
        let complete = matches!(&outcome, Ok((_, _, _, AgentLoopExit::Completed)));
        if complete || attempt >= job.max_retries {
            if !complete && attempt >= job.max_retries {
                super::retry_log::exhausted(&job.id, attempt, job.max_retries);
            }
            return (outcome, attempt);
        }
        let delay = backoff::calculate(attempt, job.base_delay_ms, job.max_delay_ms, 2.0);
        match &outcome {
            Ok((_, _, _, exit)) => super::retry_log::incomplete(
                &job.id,
                attempt,
                job.max_retries,
                *exit,
                delay.as_millis(),
            ),
            Err(error) => {
                super::retry_log::error(&job.id, attempt, job.max_retries, error, delay.as_millis())
            }
        }
        tokio::time::sleep(delay).await;
        attempt += 1;
    }
}
