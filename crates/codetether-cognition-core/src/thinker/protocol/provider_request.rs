//! Validated request construction for registry-backed cognition.

use anyhow::{Result, ensure};

use super::ThinkerConfig;
use codetether_provider_core::codex::reasoning_catalog;
use codetether_provider_core::{CompletionRequest, ContentPart, Message, Role};

///
/// # Arguments
/// * `model` - Resolved registry model identifier.
///
/// # Returns
/// Unit for a supported model.
///
/// # Errors
/// Returns an error when the model is outside the accepted GPT-5.6 class.
///
/// # Examples
/// ```
/// # use codetether_cognition_core::thinker::{ThinkerConfig, ThinkerOutput};
/// # use codetether_cognition_core::thinker::protocol::openai_wire::*;
/// # use codetether_provider_core::CompletionResponse;
/// # fn example(model: &str) {
/// let _result = codetether_cognition_core::thinker::protocol::provider_request::validate_model(model);
/// # }
/// ```
pub fn validate_model(model: &str) -> Result<()> {
    ensure!(
        reasoning_catalog::is_gpt_56(model),
        "cognition requires a GPT-5.6 class model"
    );
    Ok(())
}

///
/// # Arguments
/// * `config` - Thinker settings for the request or fallback model.
/// * `model` - Resolved registry model identifier.
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
/// # fn example( config: &ThinkerConfig, model: String, system_prompt: &str, user_prompt: &str, ) {
/// let _result = codetether_cognition_core::thinker::protocol::provider_request::build(config, model, system_prompt, user_prompt);
/// # }
/// ```
pub fn build(
    config: &ThinkerConfig,
    model: String,
    system_prompt: &str,
    user_prompt: &str,
) -> CompletionRequest {
    CompletionRequest {
        messages: vec![
            message(Role::System, system_prompt),
            message(Role::User, user_prompt),
        ],
        tools: Vec::new(),
        model,
        temperature: Some(config.temperature),
        top_p: config.top_p,
        max_tokens: Some(config.max_tokens),
        stop: Vec::new(),
    }
}

fn message(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}

#[cfg(test)]
mod tests {
    use super::validate_model;

    #[test]
    fn accepts_gpt_56_from_multiple_providers() {
        assert!(validate_model("gpt-5.6-sol-fast:max").is_ok());
        assert!(validate_model("openai.gpt-5.6-sol").is_ok());
        assert!(validate_model("claude-sonnet-4").is_err());
    }
}
