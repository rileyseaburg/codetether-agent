//! Cancellation-aware completion and shared parent-goal token accounting.

use super::state::State;
use crate::provider::{CompletionRequest, CompletionResponse};

pub(super) async fn run(
    state: &State,
    request: CompletionRequest,
) -> anyhow::Result<CompletionResponse> {
    let response = match &state.control {
        Some(control) => tokio::select! {
            biased;
            _ = control.cancelled() => anyhow::bail!("Session execution cancelled"),
            result = state.provider.complete(request) => result?,
        },
        None => state.provider.complete(request).await?,
    };
    if let Some(id) = state
        .control
        .as_ref()
        .and_then(|control| control.session_id())
    {
        let tokens = response
            .usage
            .prompt_tokens
            .saturating_add(response.usage.completion_tokens);
        crate::session::runtime::goals::record_usage(id, tokens, std::time::Duration::ZERO).await?;
    }
    Ok(response)
}
