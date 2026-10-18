//! Prepend previously committed content without altering final usage or finish reason.
use codetether_provider_core::{CompletionResponse, ContentPart};

pub(super) fn run(
    mut response: CompletionResponse,
    mut content: Vec<ContentPart>,
) -> CompletionResponse {
    content.append(&mut response.message.content);
    response.message.content = content;
    response
}
