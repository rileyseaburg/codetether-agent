//! Help navigation 2 content and presentation.
use crate::blank::blank;
use crate::cmd_row::cmd_row;
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>) {
    lines.push(cmd_row(
        "/ask",
        "",
        "Ephemeral side question (full context, no tools, not saved)",
    ));
    lines.push(cmd_row("/settings", "/set", "Settings panel"));
    lines.push(cmd_row("/new", "", "Start fresh chat buffer"));
    lines.push(cmd_row("/undo", "", "Undo last turn (accepts /undo <N>)"));
    lines.push(cmd_row(
        "/fork",
        "",
        "Fork session at current point (/fork <N> drops last N turns)",
    ));
    lines.push(blank());
}
