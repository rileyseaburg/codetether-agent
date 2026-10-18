//! Help scrolling content and presentation.
use crate::blank::blank;
use crate::heading::heading;
use crate::key_row::key_row;
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>) {
    lines.push(heading("SCROLLING"));
    lines.push(key_row("Up / Down", "Scroll messages"));
    lines.push(key_row("Shift+Up / Down", "Scroll compact tool panels"));
    lines.push(key_row("PgUp / PgDn", "Scroll by page"));
    lines.push(key_row("Mouse wheel", "Scroll the current list or view"));
    lines.push(key_row("Shift+Wheel", "Scroll compact tool panels in chat"));
    lines.push(key_row("Alt+J / Alt+K", "Chat scroll down / up"));
    lines.push(key_row("Alt+D / Alt+U", "Chat page down / up"));
    lines.push(key_row(
        "Ctrl+G / Ctrl+Shift+G",
        "Jump chat to top / bottom",
    ));
    lines.push(blank());
}
