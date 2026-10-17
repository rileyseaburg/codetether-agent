//! Which Bedrock-hosted OpenAI GPT models accept `reasoning_effort`.
//!
//! Observed live on bedrock-runtime: GPT-5.x accepts the
//! `reasoning_effort` additional field, while the GPT-6 family
//! (`gpt-6-sol`, `gpt-6.1-sol`, ...) rejects it with
//! `unknown_parameter: 'reasoning_effort'`, failing the whole request.

/// Whether to send `reasoning_effort` for this Bedrock OpenAI model ID.
///
/// # Examples
///
/// ```ignore
/// assert!(accepts_reasoning_effort("openai.gpt-5.6-sol"));
/// assert!(!accepts_reasoning_effort("global.openai.gpt-6.1-sol"));
/// ```
pub(super) fn accepts_reasoning_effort(model_id: &str) -> bool {
    let id = model_id.to_ascii_lowercase();
    id.contains("openai.gpt-") && !id.contains("openai.gpt-6")
}

#[cfg(test)]
mod tests {
    use super::accepts_reasoning_effort;

    #[test]
    fn gpt6_family_omits_reasoning_effort() {
        assert!(accepts_reasoning_effort("us.openai.gpt-5.6-sol"));
        assert!(!accepts_reasoning_effort("global.openai.gpt-6.1-sol"));
        assert!(!accepts_reasoning_effort("us.openai.gpt-6-sol"));
        assert!(!accepts_reasoning_effort("openai.gpt-6-astra"));
        assert!(!accepts_reasoning_effort("us.anthropic.claude-opus-5"));
    }
}
