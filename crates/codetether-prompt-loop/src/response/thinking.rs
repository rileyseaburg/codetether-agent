//! Detect hidden-only assistant output and supply continuation guidance.
use codetether_provider_core::ContentPart;

const NUDGE: &str = "Your last turn contained only hidden reasoning/thinking. Continue now with either visible answer text or a tool call; do not stop after thinking only.";
/// Return corrective text only for a nonempty hidden-reasoning-only response.
/// # Arguments
/// * `parts` — Content parts from the assistant response.
/// # Returns
/// The established continuation nudge, or None when visible content exists.
/// # Examples
/// ```
/// assert!(codetether_prompt_loop::response::thinking_only_nudge(&[]).is_none());
/// ```
pub fn thinking_only_nudge(parts: &[ContentPart]) -> Option<&'static str> {
    if !is_thinking_only(parts) {
        return None;
    }
    tracing::warn!("assistant turn contained only thinking; continuing");
    Some(NUDGE)
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
#[path = "thinking_tests.rs"]
mod tests;
