//! Clipboard shortcut rows for the help panel.
use ratatui::text::Line;

pub(super) fn append(lines: &mut Vec<Line<'static>>) {
    lines.push(super::key_row(
        "Ctrl+O/Ctrl+Y",
        "Copy reply; /copy tool or /copy error for raw diagnostics",
    ));
    lines.push(super::key_row(
        "Ctrl+Shift+Y",
        "Copy entire conversation transcript (clean plain text)",
    ));
}
