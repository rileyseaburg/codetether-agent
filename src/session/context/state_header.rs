//! Deterministic typed state header for compression-safe constraints.

use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

const MAX_CONSTRAINTS: usize = 8;
const EXCERPT_CHARS: usize = 120;

/// Prepend a compact `[CONTEXT STATE]` message containing hard-pinned
/// constraint turns. This is injected *after* derivation, so pins survive
/// Legacy compression, Reset summaries, and Incremental selection.
pub(super) fn prepend_state_header(session: &Session, messages: &mut Vec<Message>) {
    let constraints = pinned_constraints(session);
    if constraints.is_empty() {
        return;
    }
    let header = Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: format!(
                "[CONTEXT STATE]\nHard constraints pinned by the user; obey even if older context was compressed:\n{}",
                constraints.join("\n")
            ),
        }],
    };
    messages.insert(0, header);
}

fn pinned_constraints(session: &Session) -> Vec<String> {
    super::state_header_pins::pinned_indices(session)
        .into_iter()
        .filter_map(|idx| {
            first_text_excerpt(&session.messages[idx]).map(|s| format!("- turn {idx}: {s}"))
        })
        .take(MAX_CONSTRAINTS)
        .collect()
}

fn first_text_excerpt(msg: &Message) -> Option<String> {
    msg.content.iter().find_map(|part| match part {
        ContentPart::Text { text } if !text.trim().is_empty() => {
            let line = text.trim().lines().next().unwrap_or_default();
            let mut out: String = line.chars().take(EXCERPT_CHARS).collect();
            if line.chars().count() > EXCERPT_CHARS {
                out.push('…');
            }
            Some(out)
        }
        _ => None,
    })
}
