//! Fallback payload selection and final handoff message text.
use codetether_autochat_core::{AUTOCHAT_RLM_FALLBACK_CHARS, AUTOCHAT_RLM_THRESHOLD_CHARS};
use codetether_relay_plan::truncate_with_ellipsis;
pub(super) fn fallback(output: &str) -> String {
    if output.len() > AUTOCHAT_RLM_THRESHOLD_CHARS {
        truncate_with_ellipsis(output, AUTOCHAT_RLM_FALLBACK_CHARS)
    } else {
        output.to_string()
    }
}
pub(super) fn message(task: &str, from_agent: &str, relay_payload: &str) -> String {
    format!(
        "Relay task:\n{task}\n\nIncoming handoff from @{from_agent}:\n{relay_payload}\n\n\
             Continue the work from this handoff. Keep your response focused and provide one concrete next-step instruction for the next agent."
    )
}
