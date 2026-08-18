//! Request construction for the OpenAI-compatible thinker backend.

use super::ThinkerConfig;
use super::openai_wire::{OpenAIChatRequest, OpenAIMessage};

/// Build a non-streaming chat request for the given prompts.
pub(super) fn body(
    config: &ThinkerConfig,
    system_prompt: &str,
    user_prompt: &str,
) -> OpenAIChatRequest {
    OpenAIChatRequest {
        model: config.model.clone(),
        messages: vec![
            OpenAIMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            OpenAIMessage {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ],
        temperature: config.temperature,
        top_p: config.top_p,
        max_tokens: config.max_tokens,
        stream: false,
    }
}
