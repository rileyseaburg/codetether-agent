//! Application projection for inspector rendering.
pub mod layout;
pub mod metrics;
pub mod tool_history;
use crate::tui::app::state::{App, AppState};
/// Render the full inspector from the application values.
///
/// # Arguments
/// * `f` - Destination terminal frame.
/// * `app` - Application values projected into the shared view.
///
/// # Returns
/// Draws the requested inspector panel without changing application state.
///
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame, app: &codetether_agent::tui::app::state::App) {
/// codetether_agent::tui::ui::inspector::render_inspector_view(f, app);
/// # }
/// ```
pub fn render_inspector_view(f: &mut ratatui::Frame, app: &App) {
    layout::render_inspector(f, &app.state);
}
pub(super) fn projection(state: &AppState) -> codetether_inspector_view::InspectorView<'_> {
    codetether_inspector_view::InspectorView {
        last_completion_model: state.last_completion_model.as_deref(),
        last_completion_prompt_tokens: state.last_completion_prompt_tokens,
        last_completion_output_tokens: state.last_completion_output_tokens,
        last_completion_latency_ms: state.last_completion_latency_ms,
        last_tool_name: state.tool_timing.last_tool_name.as_deref(),
        last_tool_latency_ms: state.tool_timing.last_tool_latency_ms,
        last_tool_success: state.tool_timing.last_tool_success,
        message_count: state.messages.len(),
        processing: state.processing,
    }
}
