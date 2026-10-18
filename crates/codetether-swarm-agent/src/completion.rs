//! Cancellation-aware completion and shared parent-goal token accounting.

use super::state::State;
use codetether_provider_core::{CompletionRequest, CompletionResponse};

pub(super) async fn run<H: crate::Host>(
    state: &State<H>,
    request: CompletionRequest,
) -> anyhow::Result<CompletionResponse> {
    codetether_provider_step::controlled::complete::<H>(
        &state.inner.provider,
        request,
        state.inner.control.as_ref(),
    )
    .await
}
