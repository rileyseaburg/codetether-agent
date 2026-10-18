//! Bind shared Git presentation to the application's input preview.
use crate::tui::{app::state::git_state::GitViewState, input::render_input_preview};
use ratatui::{Frame, layout::Rect};
/// Render captured Git state with the native command preview.
pub fn render_git_view(f: &mut Frame, area: Rect, state: &GitViewState, status: &str) {
    codetether_git_view::render_git_view(f, area, state, status, render_input_preview);
}
