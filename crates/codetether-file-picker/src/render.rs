//! File picker render.
use crate::FilePickerState;

use super::types::FilePickerMode;

/// Render the directory browser or full preview selected by state.
/// # Arguments
/// * `f` - Target frame.
/// * `area` - Available screen rectangle.
/// * `state` - Current browser/viewer snapshot.
/// # Returns
/// Unit after drawing the active mode.
/// # Examples
/// ```no_run
/// # fn example(f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// let state = codetether_file_picker::FilePickerState::default();
/// codetether_file_picker::render_file_picker(f, area, &state);
/// # }
/// ```
pub fn render_file_picker(
    f: &mut ratatui::Frame,
    area: ratatui::prelude::Rect,
    state: &FilePickerState,
) {
    match state.mode {
        FilePickerMode::Browse => {
            super::render_browser::render_browser(f, area, state);
        }
        FilePickerMode::Viewer => {
            super::render_viewer::render_viewer(f, area, state);
        }
    }
}
