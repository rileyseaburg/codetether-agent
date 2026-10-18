//! Application adapters for bus status and request timing badges.
use crate::tui::app::state::App;
pub use codetether_chat_status::{format_duration_ms, format_timestamp};
use ratatui::{style::Color, text::Span};
fn bus_status(app: &App) -> codetether_chat_status::BusStatus {
    codetether_chat_status::BusStatus {
        connected: app.state.worker.a2a_connected,
        agents: app.state.worker.worker_bridge_registered_agents.len(),
        queued: app.state.worker.worker_task_queue.len(),
        processing: app
            .state
            .worker
            .worker_bridge_processing_state
            .unwrap_or(app.state.processing),
        peer_ready: app.state.peer_endpoint_ready,
    }
}
/// Return the bus label and effective display color.
pub fn bus_status_label_and_color(app: &App) -> (String, Color) {
    codetether_chat_status::bus_status_label_and_color(bus_status(app))
}
/// Render the effective agent-bus status.
pub fn bus_status_badge_span(app: &App) -> Span<'static> {
    codetether_chat_status::bus_status_badge_span(bus_status(app))
}
/// Render active or most-recent request timing values.
pub fn latency_badge_spans(app: &App) -> Option<Vec<Span<'static>>> {
    let timings = codetether_chat_status::RequestTimings {
        processing: app.state.processing,
        current_first_token_ms: app.state.request_timing.current_request_first_token_ms,
        last_first_token_ms: app.state.request_timing.last_request_first_token_ms,
        last_last_token_ms: app.state.request_timing.last_request_last_token_ms,
    };
    codetether_chat_status::latency_badge_spans(timings, || app.state.current_request_elapsed_ms())
}
/// Return a temporary model label while processing.
pub fn session_model_label(state: &crate::tui::app::state::AppState) -> Option<String> {
    codetether_chat_status::session_model_label(state.processing)
}
