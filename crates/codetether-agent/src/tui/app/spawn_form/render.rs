//! Application binding for the shared spawn form overlay.
use crate::tui::app::state::AppState;
use ratatui::Frame;
/// Render the spawn form if it is active.
pub fn render_spawn_form_if_needed(f: &mut Frame, app_state: &mut AppState) {
    codetether_spawn_form::render_spawn_form_if_needed(f, app_state.spawn_form.as_ref());
}
