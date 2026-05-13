//! Text-only fallback path and continuation prompt.

use super::super::extract::extract_final;
use crate::traits::{LlmMessage, LlmResponse};

/// Try extracting an answer from a text-only response.
pub(super) fn try_text_path(
    response: &LlmResponse,
    summary_mode: bool,
    iterations: usize,
) -> Option<String> {
    if let Some(a) = extract_final(&response.text) {
        return Some(a);
    }
    if summary_mode && !response.text.trim().is_empty() {
        return Some(response.text.clone());
    }
    if iterations >= 3 && response.text.len() > 500 && !response.text.contains("```") {
        return Some(response.text.clone());
    }
    None
}

/// Push a continuation prompt after a text-only model response.
pub(super) fn push_continuation(conversation: &mut Vec<LlmMessage>, response: &LlmResponse) {
    conversation.push(LlmMessage::assistant(response.text.clone()));
    conversation.push(LlmMessage::user(
        "Continue analysis. Call FINAL(\"your answer\") when ready.".into(),
    ));
}
