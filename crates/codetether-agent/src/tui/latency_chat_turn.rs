//! Bind native chat-turn statistics to the shared latency section.
use crate::tui::app::state::App;
use ratatui::text::Line;
/// Append completed chat-turn latency lines to the inspector.
pub(crate) fn push_chat_turn_latency(lines: &mut Vec<Line<'static>>, app: &App) {
    codetether_latency_view::push_chat_turn_latency(lines, &app.state.request_timing.chat_latency);
}
