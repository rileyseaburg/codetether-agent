//! Guard against hidden-reasoning-only assistant turns.

use super::super::Runner;
use crate::provider::{CompletionResponse, ContentPart};

const NUDGE: &str = "Your last turn contained only hidden reasoning/thinking. Continue now with either visible answer text or a tool call; do not stop after thinking only.";

pub(super) fn continue_instead(runner: &mut Runner<'_>, response: &CompletionResponse) -> bool {
    if !is_thinking_only(&response.message.content) {
        return false;
    }
    tracing::warn!("assistant turn contained only thinking; continuing");
    super::nudge::add(runner, NUDGE);
    true
}

fn is_thinking_only(parts: &[ContentPart]) -> bool {
    let mut saw_hidden = false;
    for part in parts {
        match part {
            ContentPart::Thinking { text, signature } => {
                saw_hidden |= !text.trim().is_empty() || signature.is_some();
            }
            ContentPart::Text { text } if text.trim().is_empty() => {}
            _ => return false,
        }
    }
    saw_hidden
}

#[cfg(test)]
#[path = "thinking_only_tests.rs"]
mod tests;
