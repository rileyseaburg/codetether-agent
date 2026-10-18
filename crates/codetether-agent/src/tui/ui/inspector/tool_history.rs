//! Application adapter for the shared inspector tool_history.
use crate::tui::app::state::AppState;
/// Render the inspector tool_history using borrowed application values.
///
/// # Arguments
/// * `f` - Destination terminal frame.
/// * `state` - Application values projected into the shared view.
/// * `area` - Destination panel rectangle.
///
/// # Returns
/// Draws the requested inspector panel without changing application state.
///
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame, state: &codetether_agent::tui::app::state::AppState) {
/// let area = f.area();
/// codetether_agent::tui::ui::inspector::tool_history::render_tool_history(f, state, area);
/// # }
/// ```
pub fn render_tool_history(f: &mut ratatui::Frame, state: &AppState, area: ratatui::layout::Rect) {
    codetether_inspector_view::render_tool_history(f, &super::projection(state), area);
}
