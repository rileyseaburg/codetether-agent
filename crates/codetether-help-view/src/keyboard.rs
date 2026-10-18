//! Help keyboard content and presentation.
use crate::blank::blank;
use crate::copy_shortcuts;
use crate::heading::heading;
use crate::key_row::key_row;
use crate::separator::separator;
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>) {
    // ── Keyboard shortcuts ──
    lines.push(heading("KEYBOARD SHORTCUTS"));
    lines.push(separator());
    lines.push(key_row(
        "Ctrl+C",
        "Interrupt turn (while streaming) or quit (when idle)",
    ));
    lines.push(key_row(
        "Ctrl+E",
        "Diff most recent file referenced by an agent",
    ));
    lines.push(key_row(
        "Ctrl+F",
        "Open most recent agent file in the file viewer",
    ));
    lines.push(key_row("Ctrl+Q", "Quit"));
    lines.push(key_row("Esc", "Back / close overlay / exit detail"));
    lines.push(key_row("Ctrl+T", "Symbol search (workspace)"));
    lines.push(key_row("Ctrl+W", "Start a /ask side question in chat"));
    copy_shortcuts::append(lines);
    lines.push(key_row("Ctrl+R", "Start/stop voice recording"));
    lines.push(key_row("Ctrl+V", "Paste from clipboard (or image)"));
    lines.push(key_row("Enter", "Send message or run slash command"));
    lines.push(key_row("Tab", "Accept slash autocomplete"));
    lines.push(blank());
}
