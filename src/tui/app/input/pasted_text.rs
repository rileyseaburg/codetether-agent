//! Sidecar buffer for large multi-line paste blocks.
//!
//! When a paste arrives that is large or multi-line, we don't want to
//! flood the input box with thousands of characters. Instead the full
//! content lands in [`AppState::pending_text_pastes`] and the visible
//! input gets a short placeholder like
//! `[Pasted text #1: 42 lines, 1.2 KiB]`. On submit, the placeholder
//! is expanded back to the full content for the agent prompt while
//! the user-facing chat history keeps the compact summary.

use crate::tui::app::state::AppState;

/// Threshold above which a paste is summarised instead of being
/// inlined into the input buffer.
///
/// Multi-line snippets shorter than this are still useful to see
/// inline (short addresses, two-line commands, etc.). Anything
/// longer becomes visual noise that hides the user's actual prompt
/// text.
const PASTE_SUMMARIZE_LINES: usize = 5;
const PASTE_SUMMARIZE_BYTES: usize = 400;

/// One sidecar entry for a paste that was summarised in the input
/// buffer. The `id` shows up in the placeholder rendered for the
/// user; expansion at submit time matches by that id's placeholder.
#[derive(Debug, Clone)]
pub struct PendingTextPaste {
    pub id: u32,
    pub content: String,
}

impl PendingTextPaste {
    pub fn lines(&self) -> usize {
        self.content.lines().count().max(1)
    }

    pub fn bytes(&self) -> usize {
        self.content.len()
    }

    /// User-visible compact summary that takes the place of the full
    /// pasted text in the input buffer and in the chat-history user
    /// message. The agent receives the expanded form at submit time.
    pub fn placeholder(&self) -> String {
        format!(
            "[Pasted text #{}: {} lines, {}]",
            self.id,
            self.lines(),
            format_size(self.bytes())
        )
    }
}

/// Format a byte count as a short human-readable string (e.g. `1.2 KiB`).
pub fn format_size(bytes: usize) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    if bytes < 1024 {
        format!("{bytes} B")
    } else if (bytes as f64) < MIB {
        format!("{:.1} KiB", bytes as f64 / KIB)
    } else {
        format!("{:.1} MiB", bytes as f64 / MIB)
    }
}

/// Decide whether a paste is large enough to warrant summarisation.
pub fn should_summarize(text: &str) -> bool {
    let lines = text.lines().count();
    let bytes = text.len();
    lines >= PASTE_SUMMARIZE_LINES || bytes >= PASTE_SUMMARIZE_BYTES
}

/// Allocate the next paste id.
fn next_paste_id(pastes: &[PendingTextPaste]) -> u32 {
    pastes
        .iter()
        .map(|p| p.id)
        .max()
        .map(|m| m + 1)
        .unwrap_or(1)
}

/// Push a new sidecar entry and return the placeholder string the
/// caller should insert into the input buffer at the cursor.
pub fn attach_paste(state: &mut AppState, content: String) -> String {
    let id = next_paste_id(&state.pending_text_pastes);
    let paste = PendingTextPaste { id, content };
    let placeholder = paste.placeholder();
    state.pending_text_pastes.push(paste);
    placeholder
}

/// Replace each `[Pasted text #N: …]` placeholder in `prompt` with
/// the corresponding sidecar content wrapped in delimiters that the
/// agent can recognise. Pastes whose placeholder no longer appears
/// in the prompt are dropped silently — the user must have edited
/// them out before submitting.
pub fn expand_paste_placeholders(prompt: &str, pastes: &[PendingTextPaste]) -> String {
    let mut out = prompt.to_string();
    for paste in pastes {
        let ph = paste.placeholder();
        if !out.contains(&ph) {
            continue;
        }
        let expansion = format!(
            "\n\n--- Begin pasted text #{id} ({lines} lines, {bytes} bytes) ---\n{content}\n--- End pasted text #{id} ---",
            id = paste.id,
            lines = paste.lines(),
            bytes = paste.bytes(),
            content = paste.content,
        );
        out = out.replacen(&ph, &expansion, 1);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paste(id: u32, content: &str) -> PendingTextPaste {
        PendingTextPaste {
            id,
            content: content.to_string(),
        }
    }

    #[test]
    fn format_size_human_readable() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.0 KiB");
        assert_eq!(format_size(1536), "1.5 KiB");
        assert_eq!(format_size(2 * 1024 * 1024), "2.0 MiB");
    }

    #[test]
    fn placeholder_reports_lines_and_size() {
        let p = paste(7, "a\nb\nc");
        assert_eq!(p.placeholder(), "[Pasted text #7: 3 lines, 5 B]");
    }

    #[test]
    fn placeholder_single_line_still_reports_one_line() {
        let p = paste(1, "single line");
        assert_eq!(p.lines(), 1);
        assert!(p.placeholder().contains("1 lines"));
    }

    #[test]
    fn should_summarize_thresholds() {
        // 2 lines, 22 bytes — stay inline.
        assert!(!should_summarize("first line\nsecond line"));
        // 3 lines, 28 bytes — stay inline.
        assert!(!should_summarize("line one\nline two\nline three"));
        // 5 lines — summarise.
        assert!(should_summarize("a\nb\nc\nd\ne"));
        // 1 line, 400+ bytes — summarise.
        let big = "x".repeat(400);
        assert!(should_summarize(&big));
        // 1 line, 399 bytes — stay inline.
        let almost = "x".repeat(399);
        assert!(!should_summarize(&almost));
    }

    #[test]
    fn expand_replaces_placeholder_with_full_content() {
        let pastes = vec![paste(1, "hello\nworld")];
        let prompt = "please review [Pasted text #1: 2 lines, 11 B] and respond";
        let out = expand_paste_placeholders(prompt, &pastes);
        assert!(out.contains("--- Begin pasted text #1 (2 lines, 11 bytes) ---"));
        assert!(out.contains("hello\nworld"));
        assert!(out.contains("--- End pasted text #1 ---"));
        assert!(out.starts_with("please review "));
        assert!(out.ends_with(" and respond"));
    }

    #[test]
    fn expand_drops_pastes_whose_placeholder_was_deleted() {
        let pastes = vec![paste(1, "hello"), paste(2, "world")];
        // Only #2's placeholder is present.
        let prompt = "look at [Pasted text #2: 1 lines, 5 B]";
        let out = expand_paste_placeholders(prompt, &pastes);
        assert!(out.contains("world"));
        assert!(!out.contains("hello"));
        assert!(!out.contains("#1"));
    }

    #[test]
    fn expand_replaces_only_first_occurrence_per_paste() {
        // If the user typed the same placeholder string twice, only the
        // first match expands so the sidecar content is never duplicated.
        let pastes = vec![paste(1, "DATA")];
        let prompt = "[Pasted text #1: 1 lines, 4 B] then again [Pasted text #1: 1 lines, 4 B]";
        let out = expand_paste_placeholders(prompt, &pastes);
        assert_eq!(out.matches("DATA").count(), 1);
        assert_eq!(out.matches("[Pasted text #1: 1 lines, 4 B]").count(), 1);
    }

    #[test]
    fn next_paste_id_starts_at_one_and_increments() {
        let mut pastes: Vec<PendingTextPaste> = Vec::new();
        assert_eq!(next_paste_id(&pastes), 1);
        pastes.push(paste(1, "a"));
        assert_eq!(next_paste_id(&pastes), 2);
        pastes.push(paste(2, "b"));
        assert_eq!(next_paste_id(&pastes), 3);
    }
}
