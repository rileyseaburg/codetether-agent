//! ChatGPT OAuth HTTP recovery factory.

use super::super::super::{CompletionRequest, OpenAiCodexProvider};
use super::super::{Chunks, retry};

/// Recover an interrupted WebSocket request through ChatGPT HTTP.
pub(in crate::provider::openai_codex) fn recover(
    provider: OpenAiCodexProvider,
    primary: Chunks,
    request: CompletionRequest,
    token: String,
    account: String,
) -> Chunks {
    retry_stream(provider, Some(primary), request, token, account)
}

/// Start a ChatGPT HTTP request with private transport retries.
pub(in crate::provider::openai_codex) fn start(
    provider: OpenAiCodexProvider,
    request: CompletionRequest,
    token: String,
    account: String,
) -> Chunks {
    retry_stream(provider, None, request, token, account)
}

fn retry_stream(
    provider: OpenAiCodexProvider,
    primary: Option<Chunks>,
    request: CompletionRequest,
    token: String,
    account: String,
) -> Chunks {
    retry::with_http_retry(primary, move || {
        let values = (
            provider.clone(),
            request.clone(),
            token.clone(),
            account.clone(),
        );
        async move {
            values
                .0
                .complete_stream_with_chatgpt_http_responses(values.1, values.2, values.3)
                .await
        }
    })
}
