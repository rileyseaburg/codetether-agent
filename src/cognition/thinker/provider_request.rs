//! Validated request construction for registry-backed cognition.

use anyhow::{Result, ensure};

use super::ThinkerConfig;
use crate::provider::openai_codex::reasoning_catalog;
use crate::provider::{CompletionRequest, ContentPart, Message, Role};

pub(super) fn validate_model(model: &str) -> Result<()> {
    ensure!(
        reasoning_catalog::is_gpt_56(model),
        "cognition requires a GPT-5.6 class model"
    );
    Ok(())
}

pub(super) fn build(
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
