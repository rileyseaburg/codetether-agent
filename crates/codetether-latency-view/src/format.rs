//! Shared duration, throughput and section-heading presentation.
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

#[allow(dead_code)]
pub(crate) fn section_heading(title: &str) -> Line<'static> {
    Line::from(vec![Span::styled(
        format!("  {title}"),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )])
}

#[allow(dead_code)]
pub(crate) fn format_duration(ms: u64) -> String {
    match ms {
        0..=999 => format!("{ms}ms"),
        1_000..=9_999 => format!("{:.2}s", ms as f64 / 1_000.0),
        10_000..=59_999 => format!("{:.1}s", ms as f64 / 1_000.0),
        _ => format!("{:.1}m", ms as f64 / 60_000.0),
    }
}

pub(crate) fn format_tps(tps: f64) -> String {
    if tps >= 100.0 {
        format!("{tps:.0}")
    } else if tps >= 10.0 {
        format!("{tps:.1}")
    } else {
        format!("{tps:.2}")
    }
}
