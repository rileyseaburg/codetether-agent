use crate::provider::CompletionResponse;
use anyhow::Result;

const EMPTY_STREAM_MSG: &str =
    "Provider stream ended without assistant content; no text, thinking, or tool call was emitted.";

pub(super) fn reject(response: CompletionResponse) -> Result<CompletionResponse> {
    if response.message.content.is_empty() {
        anyhow::bail!("{EMPTY_STREAM_MSG}");
    }
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::reject;
    use crate::provider::{CompletionResponse, FinishReason, Message, Role, Usage};

    #[test]
    fn empty_response_is_error() {
        let response = CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content: Vec::new(),
            },
            usage: Usage::default(),
            finish_reason: FinishReason::Stop,
        };
        assert!(reject(response).is_err());
    }
}
