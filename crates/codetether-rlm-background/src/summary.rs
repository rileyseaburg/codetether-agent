//! Background RLM summarisation body.

use std::sync::Arc;

use codetether_rlm::provider_adapter::ProviderWrap;
use codetether_rlm::router::{CrateAutoProcessContext, auto_process_with_repl};

use super::job::Job;

pub(super) async fn summarize(job: &Job) -> anyhow::Result<String> {
    let (provider, model) = (job.resolver)(
        Arc::clone(&job.provider),
        &job.model,
        &job.config,
        codetether_rlm::RlmModelPurpose::Background,
    )
    .await;
    let ctx = CrateAutoProcessContext {
        tool_id: &job.tool,
        tool_args: job.input.clone(),
        session_id: &job.session_id,
        abort: None,
        on_progress: None,
        provider: Arc::new(ProviderWrap(provider)),
        model,
        bus: None,
        trace_id: None,
        subcall_provider: None,
        subcall_model: None,
        rewriter: None,
    };
    Ok(auto_process_with_repl(&job.content, ctx, &job.config)
        .await?
        .processed)
}
