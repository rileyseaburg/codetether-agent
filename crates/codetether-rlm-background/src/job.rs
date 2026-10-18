//! Background RLM worker jobs.

use std::sync::Arc;

use codetether_provider_core::Provider;
use codetether_rlm::RlmConfig;

use super::{cache, render, status, summary};

pub(super) struct Job {
    pub key: u64,
    pub content: String,
    pub tool: String,
    pub input: serde_json::Value,
    pub session_id: String,
    pub model: String,
    pub provider: Arc<dyn Provider>,
    pub config: RlmConfig,
    pub reason: String,
    pub original_bytes: usize,
    pub notify: Option<status::Notify>,
    pub resolver: crate::Resolver,
}

pub(super) async fn run(job: Job) {
    status::progress(&job.notify, "background started", 1, 1);
    tracing::info!(target: "codetether_agent::session::helper::rlm_background::job", tool = %job.tool, bytes = job.original_bytes, "RLM background summary started");
    match summary::summarize(&job).await {
        Ok(body) => {
            let summary = render::summary(&job.tool, job.original_bytes, &job.reason, &body);
            cache::complete(job.key, summary);
            status::complete(&job.notify, true, None);
            tracing::info!(target: "codetether_agent::session::helper::rlm_background::job", tool = %job.tool, "RLM background summary cached");
        }
        Err(e) => {
            cache::fail(job.key);
            status::complete(&job.notify, false, Some(e.to_string()));
            tracing::warn!(target: "codetether_agent::session::helper::rlm_background::job", tool = %job.tool, error = %e, "RLM background summary failed");
        }
    }
}
