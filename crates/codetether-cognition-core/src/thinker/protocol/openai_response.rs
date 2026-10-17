//! Convert a decoded chat-completions payload into thinker output.
use super::{ThinkerConfig, ThinkerOutput, openai_wire::OpenAIChatResponse};
use anyhow::{Result, anyhow};

/// Convert the first response choice, preserving reasoning fallback and usage.
///
/// # Arguments
/// * `config` - Thinker settings for the request or fallback model.
/// * `payload` - Decoded OpenAI-compatible response.
///
/// # Returns
/// The constructed request or normalized thinker output.
///
/// # Errors
/// Returns an error when the response has no completion choices.
///
/// # Examples
/// ```
/// # use codetether_cognition_core::thinker::{ThinkerConfig, ThinkerOutput};
/// # use codetether_cognition_core::thinker::protocol::openai_wire::*;
/// # use codetether_provider_core::CompletionResponse;
/// # fn example(config: &ThinkerConfig, payload: OpenAIChatResponse) {
/// let _result = codetether_cognition_core::thinker::protocol::openai_response::from_payload(config, payload);
/// # }
/// ```
pub fn from_payload(config: &ThinkerConfig, payload: OpenAIChatResponse) -> Result<ThinkerOutput> {
    let choice = payload
        .choices
        .first()
        .ok_or_else(|| anyhow!("thinker response did not include choices"))?;
    let text = choice.message.extract_text();
    let usage = payload.usage.unwrap_or_default();

    Ok(ThinkerOutput {
        model: payload.model.unwrap_or_else(|| config.model.clone()),
        finish_reason: choice.finish_reason.clone(),
        text,
        prompt_tokens: usage.prompt_tokens,
        completion_tokens: usage.completion_tokens,
        total_tokens: usage.total_tokens,
        cache_read_tokens: None,
        cache_write_tokens: None,
    })
}
