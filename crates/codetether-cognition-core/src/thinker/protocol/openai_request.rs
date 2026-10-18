//! Request construction for the OpenAI-compatible thinker backend.

use super::ThinkerConfig;
use super::openai_wire::{OpenAIChatRequest, OpenAIMessage};

/// Build a non-streaming chat request for the given prompts.
///
/// # Arguments
/// * `config` - Thinker settings for the request or fallback model.
/// * `system_prompt` - Caller-owned system instructions.
/// * `user_prompt` - Caller-owned user text.
///
/// # Returns
/// The constructed request or normalized thinker output.
///
/// # Examples
/// ```
/// # use codetether_cognition_core::thinker::{ThinkerConfig, ThinkerOutput};
/// # use codetether_cognition_core::thinker::protocol::openai_wire::*;
/// # use codetether_provider_core::CompletionResponse;
/// # fn example(config: &ThinkerConfig, system_prompt: &str, user_prompt: &str) {
/// let _result = codetether_cognition_core::thinker::protocol::openai_request::body(config, system_prompt, user_prompt);
/// # }
/// ```
pub fn body(config: &ThinkerConfig, system_prompt: &str, user_prompt: &str) -> OpenAIChatRequest {
    OpenAIChatRequest {
        model: config.model.clone(),
        messages: vec![
            OpenAIMessage {
                role: "system".to_string(),
                content: codetether_provider_core::request_identity::prompt(
                    "openai-compatible",
                    &config.model,
                ),
            },
            OpenAIMessage {
                role: "system".to_string(),
                content: codetether_provider_core::request_identity::caller_prompt(system_prompt)
                    .to_string(),
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

#[cfg(test)]
#[path = "openai_request_tests.rs"]
mod tests;
