//! Chat-turn latency status badge.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::tui::app::state::App;
use crate::tui::ui::status_bar::format_duration_ms;

/// Build the idle-only chat-turn latency badge.
pub fn turn_latency_badge(app: &App) -> Option<Span<'static>> {
    if app.state.processing {
        return None;
    }
    let last = app.state.chat_latency.last_e2e_ms()?;
    let avg = app
        .state
        .chat_latency
        .avg_e2e_ms()
        .map(format_duration_ms)
        .unwrap_or_else(|| "—".to_string());
    Some(Span::styled(
        format!(" TURN {} (avg {})", format_duration_ms(last), avg),
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::DIM),
    ))
}
