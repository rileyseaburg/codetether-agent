//! Owned key/value rows for tool history.
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub(super) fn row(key: &str, val: impl AsRef<str>, color: Color, label: Style) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{key} "), label),
        Span::styled(val.as_ref().to_string(), Style::default().fg(color)),
    ])
}
