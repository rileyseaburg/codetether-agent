//! Bind shared Git presentation to the application's input preview.
use crate::GitViewState;
use codetether_tui_style::input_preview::render_input_preview;
use ratatui::{Frame, layout::Rect};
/// Render captured Git state with the native command preview.
/// # Arguments
/// Receives the target frame, panel bounds and the current view data.
/// # Returns
/// Draws the panel using the shared input-preview presentation.
/// # Examples
/// ```no_run
/// # fn draw(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// codetether_git_view::standard::render_git_view(frame, area, &Default::default(), "Ready");
/// # }
/// ```
pub fn render_git_view(f: &mut Frame, area: Rect, state: &GitViewState, status: &str) {
    crate::render_git_view(f, area, state, status, render_input_preview);
}
