//! Chat-turn latency section rendering.

use ratatui::text::Line;

use crate::tui::app::state::App;
use crate::tui::latency::{format_duration, section_heading};

/// Append completed chat-turn latency lines to the inspector.
pub(crate) fn push_chat_turn_latency(lines: &mut Vec<Line<'static>>, app: &App) {
    lines.push(Line::from(""));
    lines.push(section_heading("Chat Turn Latency (E2E)"));
    let stats = &app.state.chat_latency;
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
