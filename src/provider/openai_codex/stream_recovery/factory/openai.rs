//! OpenAI API-key HTTP recovery factory.

use super::super::super::{CompletionRequest, OpenAiCodexProvider};
use super::super::{Chunks, retry};

/// Recover an interrupted WebSocket request through OpenAI HTTP.
pub(in crate::provider::openai_codex) fn recover(
    provider: OpenAiCodexProvider,
    primary: Chunks,
    request: CompletionRequest,
    key: String,
) -> Chunks {
    retry_stream(provider, Some(primary), request, key)
}

/// Start an OpenAI HTTP request with private transport retries.
pub(in crate::provider::openai_codex) fn start(
    provider: OpenAiCodexProvider,
    request: CompletionRequest,
    key: String,
) -> Chunks {
    retry_stream(provider, None, request, key)
}

fn retry_stream(
    provider: OpenAiCodexProvider,
    primary: Option<Chunks>,
    request: CompletionRequest,
    key: String,
) -> Chunks {
    retry::with_http_retry(primary, move || {
        let values = (provider.clone(), request.clone(), key.clone());
        async move {
            values
                .0
                .complete_stream_with_openai_http_responses(values.1, values.2)
                .await
        }
    })
}
