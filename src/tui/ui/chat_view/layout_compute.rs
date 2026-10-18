//! Application visibility projection for shared chat geometry.
use super::layout_chunks::ChatChunks;
use crate::tui::app::state::App;
use ratatui::layout::Rect;
/// Compute panel rectangles using the current input and visible UI features.
///
/// # Arguments
/// * `area` - Available terminal rectangle.
/// * `app` - Input, suggestions, and agent visibility state.
///
/// # Returns
/// The message, input, status and optional header/suggestion rectangles.
///
/// # Examples
/// ```no_run
/// # fn draw(app: &codetether_agent::tui::app::state::App) {
/// let chunks = codetether_agent::tui::ui::chat_view::layout_compute::compute_chat_chunks(
///     ratatui::layout::Rect::new(0,0,80,24), app);
/// assert_eq!(chunks.messages.width, 80);
/// # }
/// ```
pub fn compute_chat_chunks(area: Rect, app: &App) -> ChatChunks {
    codetether_chat_layout::compute_chat_chunks(
        area,
        &app.state.composer.input,
        app.state
            .composer
            .slash_suggestions_visible(&app.state.view_mode, app.state.processing),
        super::agent_bar::presence::visible(app),
    )
}
#[cfg(test)]
#[path = "layout_compute_tests.rs"]
mod tests;
