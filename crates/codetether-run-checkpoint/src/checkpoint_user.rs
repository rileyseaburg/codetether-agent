//! Retain human corrections without confusing runtime continuations with input.
use super::checkpoint_walk::ScanState;
use codetether_provider_core::ContentPart;

pub(super) fn scan(parts: &[ContentPart], state: &mut ScanState) {
    let text = parts
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    if text.trim().is_empty()
        || (text.starts_with("Continue working toward the active thread goal.")
            && text.contains("<objective>"))
        || text.starts_with("Continue this live CodeTether run from its structured checkpoint.")
    {
        return;
    }
    state.user_context.push(crate::UserEvidence {
        source_turn: state.source_turn,
        text,
    });
    state.next_action = Some(format!(
        "Apply the latest user instruction at turn {} before following earlier assistant proposals.",
        state.source_turn
    ));
}
