//! Metrics wrapping for scoped and unscoped provider streams.

use super::super::super::{CompletionRequest, StreamChunk};
use super::super::MetricsProvider;
use anyhow::Result;

pub(in super::super) async fn stream(
    provider: &MetricsProvider,
    request: CompletionRequest,
) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
    run(provider, request, None).await
}

pub(in super::super) async fn stream_scoped(
    provider: &MetricsProvider,
    request: CompletionRequest,
    session_id: &str,
) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
    run(provider, request, Some(session_id)).await
}

async fn run(
    provider: &MetricsProvider,
    request: CompletionRequest,
    session_id: Option<&str>,
) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
    let model = request.model.clone();
    let name = provider.inner.name().to_string();
    let start = std::time::Instant::now();
    let result = match session_id {
        Some(id) => provider.inner.complete_stream_scoped(request, id).await,
        None => provider.inner.complete_stream(request).await,
    };
    match result {
        Ok(inner) => Ok(Box::pin(super::super::stream::MetricsStream::new(
            inner, name, model, start,
        ))),
        Err(error) => {
            super::super::record::stream(&name, &model, start, None, None, false).await;
            Err(error)
        }
    }
}
