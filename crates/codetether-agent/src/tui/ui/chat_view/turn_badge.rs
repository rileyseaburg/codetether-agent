//! Lazy turn-latency projection for the shared badge.
use crate::tui::app::state::App;
use ratatui::text::Span;
/// Render the completed-turn latency badge while idle.
///
/// # Arguments
/// * `app` - Application values supplied to the shared renderer.
///
/// # Returns
/// Owned status display spans, or no badge when inactive.
///
/// # Examples
/// ```no_run
/// # fn example(app: &codetether_agent::tui::app::state::App) {
/// let _display = codetether_agent::tui::ui::chat_view::turn_badge::turn_latency_badge(app);
/// # }
/// ```
pub fn turn_latency_badge(app: &App) -> Option<Span<'static>> {
    codetether_chat_status::turn_latency_badge(
        app.state.processing,
        || app.state.request_timing.chat_latency.last_e2e_ms(),
        || app.state.request_timing.chat_latency.avg_e2e_ms(),
    )
}
