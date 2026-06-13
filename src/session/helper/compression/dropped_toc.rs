//! Compact table-of-contents for dropped transcript ranges.
//!
//! Rendered into `[AUTO CONTEXT COMPRESSION]` and `[CONTEXT RESET]`
//! markers so the model knows *what* was dropped and which turn index
//! to pass to `context_browse` / `session_recall` — indexed lookup
//! instead of blind search.

use crate::provider::{ContentPart, Message, Role};

const MAX_ENTRIES: usize = 10;
const EXCERPT_CHARS: usize = 70;

/// Render a navigable index of the user turns inside a dropped prefix.
///
/// `base_index` is the transcript index of `prefix[0]`, so rendered
/// turn numbers line up with `context_browse show_turn`. Returns an
/// empty string when the prefix contains no user text.
pub(crate) fn render_toc(prefix: &[Message], base_index: usize) -> String {
    let mut lines: Vec<String> = prefix
        .iter()
        .enumerate()
        .filter(|(_, m)| matches!(m.role, Role::User))
        .filter_map(|(i, m)| {
            first_text_excerpt(m).map(|e| format!("  [turn {}] {e}", base_index + i))
        })
        .collect();
    if lines.is_empty() {
        return String::new();
    }
    if lines.len() > MAX_ENTRIES {
        let skipped = lines.len() - MAX_ENTRIES;
        lines.truncate(MAX_ENTRIES);
        lines.push(format!("  … plus {skipped} more user turns"));
    }
    format!(
        "\n\n[DROPPED-RANGE INDEX] turns {}-{} were compressed. User turns in that range:\n{}\n\
         Retrieve any of them via `context_browse` (action=show_turn, turn=N) or a targeted `session_recall` query.",
        base_index,
        base_index + prefix.len().saturating_sub(1),
        lines.join("\n")
    )
}

fn first_text_excerpt(msg: &Message) -> Option<String> {
    msg.content.iter().find_map(|p| match p {
        ContentPart::Text { text } if !text.trim().is_empty() => {
            let line = text.trim().lines().next().unwrap_or_default();
            let mut s: String = line.chars().take(EXCERPT_CHARS).collect();
            if line.chars().count() > EXCERPT_CHARS {
                s.push('…');
            }
            Some(s)
        }
        _ => None,
    })
}
