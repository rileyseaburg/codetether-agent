//! OpenAI API-key HTTP recovery factory.

use super::super::super::{CompletionRequest, OpenAiCodexProvider};
use super::super::{Chunks, retry};

/// Start an OpenAI HTTP request with request-opening retries.
pub(in crate::provider::openai_codex) fn start(
    provider: OpenAiCodexProvider,
    request: CompletionRequest,
    key: String,
    session_id: String,
) -> Chunks {
    retry_stream(provider, request, key, session_id)
}

fn retry_stream(
    provider: OpenAiCodexProvider,
    request: CompletionRequest,
    key: String,
    session_id: String,
) -> Chunks {
    retry::with_http_retry(move || {
        let values = (
            provider.clone(),
            request.clone(),
            key.clone(),
            session_id.clone(),
        );
        async move {
            values
                .0
                .complete_stream_with_openai_http_responses(values.1, values.2, &values.3)
                .await
        }
    })
}
