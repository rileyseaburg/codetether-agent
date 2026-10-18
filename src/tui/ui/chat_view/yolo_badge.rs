//! Application-value adapter for the shared yolo_badge status renderer.
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
/// let _display = codetether_agent::tui::ui::chat_view::yolo_badge::yolo_badge(app);
/// # }
/// ```
pub fn yolo_badge(app: &App) -> Option<Span<'static>> {
    codetether_chat_status::yolo_badge::yolo_badge(app.state.auto_apply_edits)
}
