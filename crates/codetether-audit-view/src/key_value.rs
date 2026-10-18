//! Audit view key_value rendering.
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub(crate) fn kv_line(key: &str, value: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{key:>12}: "), Style::default().fg(Color::DarkGray)),
        Span::raw(value.to_string()),
    ])
}
