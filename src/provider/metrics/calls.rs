//! Timed completion-call orchestration for [`MetricsProvider`].

use super::super::{CompletionRequest, CompletionResponse, StreamChunk, Usage};
use super::MetricsProvider;
use anyhow::Result;

/// Executes and records one non-streaming completion.
pub(super) async fn complete(
    provider: &MetricsProvider,
    request: CompletionRequest,
) -> Result<CompletionResponse> {
    let model = request.model.clone();
    let start = std::time::Instant::now();
    match provider.inner.complete(request).await {
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

/// Starts one streaming completion and installs first-token instrumentation.
pub(super) async fn stream(
    provider: &MetricsProvider,
    request: CompletionRequest,
) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
    let model = request.model.clone();
    let name = provider.inner.name().to_string();
    let start = std::time::Instant::now();
    match provider.inner.complete_stream(request).await {
        Ok(inner) => Ok(Box::pin(super::stream::MetricsStream::new(
            inner, name, model, start,
        ))),
        Err(error) => {
            super::record::stream(&name, &model, start, None, None, false).await;
            Err(error)
        }
    }
}
