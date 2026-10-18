//! Application adapter for the shared inspector layout.
use crate::tui::app::state::AppState;
/// Render the inspector layout using borrowed application values.
///
/// # Arguments
/// * `f` - Destination terminal frame.
/// * `state` - Application values projected into the shared view.
///
/// # Returns
/// Draws the requested inspector panel without changing application state.
///
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame, state: &codetether_agent::tui::app::state::AppState) {
/// codetether_agent::tui::ui::inspector::layout::render_inspector(f, state);
/// # }
/// ```
pub fn render_inspector(f: &mut ratatui::Frame, state: &AppState) {
    codetether_inspector_view::render_inspector(f, &super::projection(state));
}
