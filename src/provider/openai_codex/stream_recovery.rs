//! Transparent HTTP recovery for an interrupted Codex WebSocket attempt.

use anyhow::Result;
use futures::{StreamExt, stream::BoxStream};
use std::future::Future;

use super::{CompletionRequest, OpenAiCodexProvider, StreamChunk};

#[path = "stream_recovery/request_anchor.rs"]
pub(super) mod request_anchor;

type Chunks = BoxStream<'static, StreamChunk>;

/// Recover an interrupted ChatGPT-backed attempt over HTTP.
pub(super) fn chatgpt(
    provider: OpenAiCodexProvider,
    primary: Chunks,
    request: CompletionRequest,
    token: String,
    account: String,
) -> Chunks {
    with_http_retry(primary, move || async move {
        provider
            .complete_stream_with_chatgpt_http_responses(request, token, account)
            .await
    })
}

/// Recover an interrupted API-key-backed attempt over HTTP.
pub(super) fn openai(
    provider: OpenAiCodexProvider,
    primary: Chunks,
    request: CompletionRequest,
    key: String,
) -> Chunks {
    with_http_retry(primary, move || async move {
        provider
            .complete_stream_with_openai_http_responses(request, key)
            .await
    })
}

/// Use `retry` only when the transactional WebSocket attempt emits nothing.
pub(super) fn with_http_retry<F, Fut>(mut primary: Chunks, retry: F) -> Chunks
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = Result<Chunks>> + Send + 'static,
{
    Box::pin(async_stream::stream! {
        if let Some(first) = primary.next().await {
            yield first;
            while let Some(chunk) = primary.next().await { yield chunk; }
            return;
        }
        match retry().await {
            Ok(mut recovered) => while let Some(chunk) = recovered.next().await { yield chunk; },
            Err(error) => yield StreamChunk::Error(error.to_string()),
        }
    })
}
