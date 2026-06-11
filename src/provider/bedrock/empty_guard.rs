//! Guard against invisible-output completions.
//!
//! Encrypted-reasoning models (Claude Fable 5, Opus 4.7) can spend the whole
//! `maxTokens` budget on signature-only `reasoningContent` blocks. The
//! Converse response then has **no visible content** and
//! `stopReason: "max_tokens"`. Persisting that as an empty assistant message
//! silently eats tokens, so callers must treat it as an error.

use crate::provider::ContentPart;
use anyhow::Result;

/// Error text for completions that hit `max_tokens` with no visible output.
pub const EMPTY_MAX_TOKENS_MSG: &str = "Bedrock hit max_tokens before any visible output: the model's encrypted \
     reasoning consumed the whole output budget. Raise CODETETHER_SESSION_MAX_TOKENS \
     or simplify the request.";

/// Fail when a completion stopped at `max_tokens` without any visible
/// content (text or tool calls).
///
/// # Errors
///
/// Returns [`anyhow::Error`] when `content` is empty (or thinking-only) and
/// `stop_reason` is `max_tokens`.
pub fn check_visible_output(content: &[ContentPart], stop_reason: Option<&str>) -> Result<()> {
    let has_visible = content
        .iter()
        .any(|p| matches!(p, ContentPart::Text { .. } | ContentPart::ToolCall { .. }));
    if !has_visible && stop_reason == Some("max_tokens") {
        anyhow::bail!("{EMPTY_MAX_TOKENS_MSG}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::check_visible_output;
    use crate::provider::ContentPart;

    #[test]
    fn empty_max_tokens_completion_is_error() {
        assert!(check_visible_output(&[], Some("max_tokens")).is_err());
        let thinking_only = vec![ContentPart::Thinking {
            text: "t".into(),
            signature: None,
        }];
        assert!(check_visible_output(&thinking_only, Some("max_tokens")).is_err());
    }

    #[test]
    fn visible_or_natural_stop_is_ok() {
        let text = vec![ContentPart::Text { text: "hi".into() }];
        assert!(check_visible_output(&text, Some("max_tokens")).is_ok());
        assert!(check_visible_output(&[], Some("end_turn")).is_ok());
    }
}
