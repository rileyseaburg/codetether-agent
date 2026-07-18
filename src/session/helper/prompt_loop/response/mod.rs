//! Completion normalization and usage reporting.

mod assistant_bus;
mod build_exhaustion;
pub(super) mod build_guard;
mod flow;
mod goal;
mod native_guard;
mod output;
mod progress;
mod repeat;
mod terminal;
mod truncation;
mod usage;

use super::Runner;
use crate::provider::CompletionResponse;

pub(super) use flow::handle;

pub(super) async fn continue_goal(runner: &mut Runner<'_>) -> super::state::StepFlow {
    goal::flow(runner).await
}

pub(super) async fn usage(
    runner: &mut Runner<'_>,
    response: &CompletionResponse,
    elapsed: std::time::Duration,
) {
    usage::record(runner, response, elapsed).await;
}

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
