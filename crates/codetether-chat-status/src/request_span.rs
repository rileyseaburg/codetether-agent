//! Labelled request duration or pending marker.
use crate::format_duration_ms;
use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

pub(super) fn request_timing_span(
    label: &str,
    duration_ms: Option<u64>,
    color: Color,
    emphasize: bool,
) -> Span<'static> {
    let display = duration_ms
        .map(format_duration_ms)
        .unwrap_or_else(|| "…".to_string());
    let mut style = Style::default().fg(color);
    if emphasize {
        style = style.add_modifier(Modifier::BOLD);
    }
    Span::styled(format!(" {label} {display} "), style)
}
