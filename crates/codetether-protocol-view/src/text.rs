//! Shared field emphasis and capability labels.
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

pub(crate) fn field(label: &str, value: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            label.to_string(),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::raw(value),
    ])
}

pub(crate) fn yes_no(value: bool) -> String {
    if value { "yes" } else { "no" }.to_string()
}
