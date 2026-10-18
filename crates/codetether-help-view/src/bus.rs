//! Help bus content and presentation.
use crate::blank::blank;
use crate::heading::heading;
use crate::key_row::key_row;
use crate::separator::separator;
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>) {
    lines.push(heading("PROTOCOL BUS LOG"));
    lines.push(separator());
    lines.push(key_row("Up / Down", "Navigate entries"));
    lines.push(key_row("Enter", "Open detail view"));
    lines.push(key_row("/", "Enter filter mode"));
    lines.push(key_row("c", "Clear filter"));
    lines.push(key_row("g", "Jump to latest entry"));
    lines.push(key_row("Esc", "Close detail or filter"));
    lines.push(blank());
}
