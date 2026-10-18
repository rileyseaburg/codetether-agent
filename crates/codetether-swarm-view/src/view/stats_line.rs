//! Swarm stats line presentation.
use codetether_swarm_core::SwarmStats;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(super) fn line(stats: &SwarmStats) -> Line<'static> {
    Line::from(vec![
        Span::styled("Time: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:.1}s", stats.execution_time_ms as f64 / 1000.0),
            Style::default().fg(Color::White),
        ),
        Span::raw("  "),
        Span::styled("Speedup: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:.1}x", stats.speedup_factor),
            Style::default().fg(Color::Green),
        ),
        Span::raw("  "),
        Span::styled("Tool calls: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{}", stats.total_tool_calls),
            Style::default().fg(Color::White),
        ),
        Span::raw("  "),
        Span::styled("Critical path: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{}", stats.critical_path_length),
            Style::default().fg(Color::White),
        ),
    ])
}
