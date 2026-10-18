//! Application-value adapter for the shared status_text status renderer.
use crate::tui::app::state::App;
use ratatui::text::Span;
/// Render the status component from current application values.
///
/// # Arguments
/// * `app` - Application values used for this status component.
///
/// # Returns
/// The component's owned spans, or no badge when inactive.
///
/// # Examples
/// ```no_run
/// # fn example(app: &codetether_agent::tui::app::state::App) {
/// let _display = codetether_agent::tui::ui::chat_view::status_text::status_text_span(app);
/// # }
/// ```
pub fn status_text_span(app: &App) -> Span<'static> {
    codetether_chat_status::status_text::status_text_span(&app.state.status, app.state.processing)
}
