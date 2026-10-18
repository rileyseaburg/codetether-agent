//! Help session picker content and presentation.
use crate::blank::blank;
use crate::heading::heading;
use crate::key_row::key_row;
use crate::separator::separator;
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>) {
    // ── View-specific controls ──
    lines.push(heading("SESSION PICKER"));
    lines.push(separator());
    lines.push(key_row("Up / Down", "Navigate sessions"));
    lines.push(key_row("Enter", "Load selected session"));
    lines.push(key_row("Type", "Filter sessions by name/ID"));
    lines.push(key_row("Backspace", "Clear filter character"));
    lines.push(key_row("Esc", "Close picker"));
    lines.push(blank());
}
