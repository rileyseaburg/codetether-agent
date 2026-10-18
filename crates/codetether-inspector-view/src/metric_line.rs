//! Consistent key/value metrics and missing-value formatting.
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub(super) fn metric_line(key: &str, val: &str, color: Color, label: Style) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{key} "), label),
        Span::styled(val.to_string(), Style::default().fg(color)),
    ])
}

pub(super) fn fmt_opt(opt: Option<String>) -> String {
    opt.unwrap_or_else(|| "—".into())
}
