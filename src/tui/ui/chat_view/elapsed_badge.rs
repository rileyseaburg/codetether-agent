//! Current elapsed-time projection for the shared heat badge.
use crate::tui::app::state::AppState;
use ratatui::text::Span;
/// Render the current request elapsed time.
///
/// # Arguments
/// * `state` - Application values supplied to the shared renderer.
///
/// # Returns
/// Owned status display spans, or no badge when inactive.
///
/// # Examples
/// ```no_run
/// # fn example(state: &codetether_agent::tui::app::state::AppState) {
/// let _display = codetether_agent::tui::ui::chat_view::elapsed_badge::elapsed_badge(state);
/// # }
/// ```
pub fn elapsed_badge(state: &AppState) -> Span<'static> {
    codetether_chat_status::elapsed_badge(state.current_request_elapsed_ms())
}
