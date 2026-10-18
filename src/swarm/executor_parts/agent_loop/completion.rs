//! Cancellation-aware completion and shared parent-goal token accounting.

use super::state::State;
use crate::provider::{CompletionRequest, CompletionResponse};

struct Native;
impl codetether_provider_step::controlled::Accounting for Native {
    async fn record(
        session_id: &str,
        tokens: usize,
        elapsed: std::time::Duration,
    ) -> anyhow::Result<()> {
        crate::session::runtime::goals::record_usage(session_id, tokens, elapsed).await
    }
}

pub(super) async fn run(
    state: &State,
    request: CompletionRequest,
) -> anyhow::Result<CompletionResponse> {
    codetether_provider_step::controlled::complete::<Native>(
        &state.provider,
        request,
        state.control.as_ref(),
    )
    .await
}
