//! Completed chat-turn latency section.
use crate::format::{format_duration, section_heading};
use codetether_telemetry::chat_latency::ChatLatencyStats;
use ratatui::text::Line;

/// Append completed chat-turn latency lines to the inspector.
/// # Arguments
/// * `lines` - Rendered lines to extend.
/// * `stats` - Completed-turn timing statistics.
/// # Returns
/// Unit after appending the section heading and measured durations.
/// # Examples
/// ```
/// let stats = codetether_telemetry::chat_latency::ChatLatencyStats::default();
/// let mut lines = Vec::new();
/// codetether_latency_view::push_chat_turn_latency(&mut lines, &stats);
/// assert!(!lines.is_empty());
/// ```
pub fn push_chat_turn_latency(lines: &mut Vec<Line<'static>>, stats: &ChatLatencyStats) {
    lines.push(Line::from(""));
    lines.push(section_heading("Chat Turn Latency (E2E)"));
    if stats.count() == 0 {
        lines.push(Line::from("  No completed turns yet."));
        return;
    }
    lines.push(Line::from(format!(
        "  Turns: {} | Last: {} | Avg: {}",
        stats.count(),
        format_duration(stats.last_e2e_ms().unwrap_or(0)),
        format_duration(stats.avg_e2e_ms().unwrap_or(0)),
    )));
    lines.push(Line::from(format!(
        "  Min: {} | Max: {} | P95: {}",
        format_duration(stats.min_e2e_ms().unwrap_or(0)),
        format_duration(stats.max_e2e_ms().unwrap_or(0)),
        format_duration(stats.p95_e2e_ms().unwrap_or(0)),
    )));
    if let Some(avg_ttft) = stats.avg_ttft_ms() {
        lines.push(Line::from(format!(
            "  Avg TTFT: {}",
            format_duration(avg_ttft),
        )));
    }
}
