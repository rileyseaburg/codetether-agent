//! Shared helper for flattening rendered lines back into plain text.

use ratatui::text::Line;

/// Join the text content of every span, one row per line.
pub fn flatten(lines: &[Line<'static>]) -> String {
    lines
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.as_ref())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}
