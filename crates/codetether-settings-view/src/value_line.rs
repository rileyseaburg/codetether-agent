//! Settings value line.
use super::row_style::row_style;
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
};

pub(super) fn value_line(label: &'static str, value: String, selected: bool) -> Line<'static> {
    let prefix = if selected { "> " } else { "  " };
    let style = row_style(selected);
    Line::from(vec![
        Span::styled(prefix, style),
        Span::styled(format!("{label}: "), style),
        Span::styled(value, Style::default().fg(Color::Cyan).bold()),
    ])
}
