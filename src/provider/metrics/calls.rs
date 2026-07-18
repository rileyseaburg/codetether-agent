//! Timed completion-call orchestration for [`MetricsProvider`].

use super::super::{CompletionRequest, CompletionResponse, Usage};
use super::MetricsProvider;
use anyhow::Result;

/// Executes and records one non-streaming completion.
pub(super) async fn complete(
    provider: &MetricsProvider,
    request: CompletionRequest,
) -> Result<CompletionResponse> {
    complete_inner(provider, request, None).await
}

pub(super) async fn complete_inner(
    provider: &MetricsProvider,
    request: CompletionRequest,
    session_id: Option<&str>,
) -> Result<CompletionResponse> {
    let model = request.model.clone();
    let start = std::time::Instant::now();
    let result = match session_id {
        Some(id) => provider.inner.complete_scoped(request, id).await,
        None => provider.inner.complete(request).await,
    };
    match result {
        Ok(response) => {
            super::record::request(provider.inner.name(), &model, start, &response.usage, true)
                .await;
            Ok(response)
        }
        Err(error) => {
            super::record::request(
                provider.inner.name(),
                &model,
                start,
                &Usage::default(),
                false,
            )
            .await;
            Err(error)
        }
    }
}

#[path = "calls/scoped.rs"]
mod scoped;
#[path = "calls/stream.rs"]
mod stream_calls;
pub(super) use scoped::complete_scoped;
pub(super) use stream_calls::{stream, stream_scoped};
