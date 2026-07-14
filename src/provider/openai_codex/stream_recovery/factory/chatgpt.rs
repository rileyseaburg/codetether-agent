//! ChatGPT OAuth HTTP recovery factory.

use super::super::super::{CompletionRequest, OpenAiCodexProvider};
use super::super::{Chunks, retry};

pub(in crate::provider::openai_codex) fn recover(
    provider: OpenAiCodexProvider,
    primary: Chunks,
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

pub(in crate::provider::openai_codex) fn start(
    provider: OpenAiCodexProvider,
    request: CompletionRequest,
    token: String,
    account: String,
) -> Chunks {
    recover(
        provider,
        Box::pin(futures::stream::empty()),
        request,
        token,
        account,
    )
}
