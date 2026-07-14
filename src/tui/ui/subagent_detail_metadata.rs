//! Identity and navigation header rows for agent detail panes.

use ratatui::style::Stylize;
use ratatui::text::Line;

pub(super) fn lines(
    name: &str,
    parent: &str,
    status: &str,
    model: &str,
    mission: &str,
) -> Vec<Line<'static>> {
    vec![
        Line::from(format!("@{name} · {status}").cyan().bold()),
        Line::from(format!("parent: {parent} · model: {model}").dim()),
        Line::from(format!("mission: {mission}")),
        Line::from("Tab: next child · Esc: dashboard · ↑↓/PgUp/PgDn: scroll".dim()),
        Line::from(""),
    ]
}
