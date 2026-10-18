//! Background RLM summarisation body.

use std::sync::Arc;

use codetether_rlm::RlmRouter;
use codetether_rlm::router::AutoProcessContext;

use super::job::Job;

pub(super) async fn summarize(job: &Job) -> anyhow::Result<String> {
    let (provider, model) = (job.resolver)(
        Arc::clone(&job.provider),
        &job.model,
        &job.config,
        codetether_rlm::RlmModelPurpose::Background,
    )
    .await;
    let ctx = AutoProcessContext {
        tool_id: &job.tool,
        tool_args: job.input.clone(),
        session_id: &job.session_id,
        abort: None,
        on_progress: None,
        provider,
        model,
        bus: None,
        trace_id: None,
        subcall_provider: None,
        subcall_model: None,
    };
    Ok(RlmRouter::auto_process(&job.content, ctx, &job.config)
        .await?
        .processed)
}
