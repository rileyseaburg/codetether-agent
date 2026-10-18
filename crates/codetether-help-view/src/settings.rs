//! Help settings content and presentation.
use crate::blank::blank;
use crate::heading::heading;
use crate::key_row::key_row;
use crate::separator::separator;
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>) {
    lines.push(heading("SETTINGS"));
    lines.push(separator());
    lines.push(key_row("Up / Down", "Select a setting"));
    lines.push(key_row("Enter", "Toggle selected setting"));
    lines.push(key_row("a", "Toggle edit auto-apply"));
    lines.push(key_row("n", "Toggle network access"));
    lines.push(key_row("Tab", "Toggle slash autocomplete"));
    lines.push(key_row("Esc", "Return to chat"));
    lines.push(blank());
}
