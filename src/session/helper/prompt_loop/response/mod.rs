//! Completion normalization and usage reporting.

mod build_exhaustion;
pub(super) mod build_guard;
mod flow;
mod native_guard;
mod output;
mod progress;
mod repeat;
mod terminal;
mod truncation;

use super::Runner;
use crate::provider::CompletionResponse;
use crate::session::SessionEvent;
use std::time::Duration;

pub(super) use flow::handle;

/// Normalizes textual or router-formatted tool calls in a completion.
pub(super) async fn normalize(
    runner: &Runner<'_>,
    response: CompletionResponse,
) -> CompletionResponse {
    let response = if let Some(router) = &runner.router {
        router
            .maybe_reformat(response, &runner.model.tools, runner.model.supports_tools)
            .await
    } else {
        response
    };
    super::super::markup::normalize_textual_tool_calls(response, &runner.model.tools)
}

/// Records provider usage and emits a streaming usage report when enabled.
pub(super) async fn usage(runner: &Runner<'_>, response: &CompletionResponse, elapsed: Duration) {
    super::super::usage_record::record_step_usage(
        &runner.model.provider_name,
        &runner.model.model_id,
        &response.usage,
    );
    let Some(tx) = &runner.events else {
        return;
    };
    let _ = tx
        .send(SessionEvent::UsageReport {
            prompt_tokens: response.usage.prompt_tokens,
            completion_tokens: response.usage.completion_tokens,
            duration_ms: elapsed.as_millis() as u64,
            model: runner.model.model_id.clone(),
        })
        .await;
}
