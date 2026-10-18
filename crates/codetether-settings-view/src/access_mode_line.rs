//! Settings access mode line.
use super::value_line::value_line;
use ratatui::text::Line;

pub(super) fn access_mode_line(selected: bool, value: String) -> Line<'static> {
    value_line("Access mode", value, selected)
}
