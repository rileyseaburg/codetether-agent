//! OpenAI API-key HTTP recovery factory.

use super::super::super::{CompletionRequest, OpenAiCodexProvider};
use super::super::{Chunks, retry};

pub(in crate::provider::openai_codex) fn recover(
    provider: OpenAiCodexProvider,
    primary: Chunks,
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

pub(in crate::provider::openai_codex) fn start(
    provider: OpenAiCodexProvider,
    request: CompletionRequest,
    key: String,
) -> Chunks {
    recover(provider, Box::pin(futures::stream::empty()), request, key)
}
