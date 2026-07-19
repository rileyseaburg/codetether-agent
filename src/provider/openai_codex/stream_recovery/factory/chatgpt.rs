//! ChatGPT OAuth HTTP recovery factory.

use anyhow::Context;

use super::super::super::{CompletionRequest, OpenAiCodexProvider};
use super::super::{Chunks, retry};

/// Start a ChatGPT HTTP request with request-opening retries.
pub(in crate::provider::openai_codex) fn start(
    provider: OpenAiCodexProvider,
    request: CompletionRequest,
    session_id: String,
) -> Chunks {
    retry_stream(provider, request, session_id)
}

fn retry_stream(
    provider: OpenAiCodexProvider,
    request: CompletionRequest,
    session_id: String,
) -> Chunks {
    retry::with_http_retry(move || {
        let values = (provider.clone(), request.clone(), session_id.clone());
        async move {
            let token = values.0.get_access_token().await?;
            let account = values
                .0
                .resolved_chatgpt_account_id(&token)
                .context("OpenAI Codex token is missing ChatGPT account ID")?;
            values
                .0
                .complete_stream_with_chatgpt_http_responses(values.1, token, account, &values.2)
                .await
        }
    })
}
