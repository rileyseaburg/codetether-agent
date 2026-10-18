//! Help text editing content and presentation.
use crate::blank::blank;
use crate::heading::heading;
use crate::key_row::key_row;
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>) {
    lines.push(heading("TEXT EDITING"));
    lines.push(key_row("Left / Right", "Move cursor"));
    lines.push(key_row("Ctrl+Left/Right", "Move by word"));
    lines.push(key_row("Home / End", "Jump to start / end of input"));
    lines.push(key_row("Backspace", "Delete backward"));
    lines.push(key_row("Delete", "Delete forward"));
    lines.push(key_row("Ctrl+Up / Down", "Command history (in chat)"));
    lines.push(blank());
}
