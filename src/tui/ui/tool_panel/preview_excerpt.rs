//! Truncate-and-line-wrap helper backing [`super::preview::push_preview_lines`].

use crate::tui::app::text::truncate_preview;

const TOOL_PANEL_ITEM_MAX_LINES: usize = 18;
const TOOL_PANEL_ITEM_MAX_BYTES: usize = 6_000;

pub(super) struct PreviewExcerpt {
    pub lines: Vec<String>,
    pub truncated: bool,
}

pub(super) fn preview_excerpt(text: &str, preview_width: usize) -> PreviewExcerpt {
    let truncated_bytes = truncate_at_char_boundary(text, TOOL_PANEL_ITEM_MAX_BYTES);
    let bytes_truncated = truncated_bytes.len() < text.len();
    let mut lines = Vec::new();
    let mut remaining = truncated_bytes.lines();

    for line in remaining.by_ref().take(TOOL_PANEL_ITEM_MAX_LINES) {
        lines.push(truncate_preview(line, preview_width));
    }

    PreviewExcerpt {
        lines,
        truncated: bytes_truncated || remaining.next().is_some(),
    }
}

fn truncate_at_char_boundary(text: &str, max_bytes: usize) -> &str {
    if text.len() <= max_bytes {
        return text;
    }
    let mut cutoff = 0;
    for (idx, ch) in text.char_indices() {
        let next = idx + ch.len_utf8();
        if next > max_bytes {
            break;
        }
        cutoff = next;
    }
    &text[..cutoff]
}
