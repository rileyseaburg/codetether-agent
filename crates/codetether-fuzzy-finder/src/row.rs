//! Fuzzy finder row presentation.
use ratatui::{
    style::{Color, Modifier, Style},
    text::Line,
};

pub(crate) fn result_line(selected: bool, display: &str) -> Line<'static> {
    let prefix = if selected { "▶ " } else { "  " };
    let style = if selected {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    Line::styled(format!("{prefix}{display}"), style)
}
