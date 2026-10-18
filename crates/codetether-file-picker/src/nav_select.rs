//! File picker nav select operations.
use crate::FilePickerState;

use super::preview_cache::refresh_preview;
use super::types::FilePickerMode;

/// Select the previous entry in browser mode and refresh its preview.
/// # Arguments
/// * `state` - Mutable browser/viewer state.
/// # Returns
/// Unit after applying the action if the current mode permits it.
/// # Examples
/// ```no_run
/// let mut state = codetether_file_picker::FilePickerState::default();
/// codetether_file_picker::file_picker_select_prev(&mut state);
/// ```
pub fn file_picker_select_prev(state: &mut FilePickerState) {
    if state.mode == FilePickerMode::Browse && state.selected > 0 {
        state.selected -= 1;
        refresh_preview(state);
    }
}

/// Select the next entry in browser mode and refresh its preview.
/// # Arguments
/// * `state` - Mutable browser/viewer state.
/// # Returns
/// Unit after applying the action if the current mode permits it.
/// # Examples
/// ```no_run
/// let mut state = codetether_file_picker::FilePickerState::default();
/// codetether_file_picker::file_picker_select_next(&mut state);
/// ```
pub fn file_picker_select_next(state: &mut FilePickerState) {
    let picker = state;
    if picker.mode == FilePickerMode::Browse && picker.selected + 1 < picker.entries.len() {
        picker.selected += 1;
        refresh_preview(picker);
    }
}

/// Scroll the full preview upward by ten lines.
/// # Arguments
/// * `state` - Mutable browser/viewer state.
/// # Returns
/// Unit after applying the action if the current mode permits it.
/// # Examples
/// ```no_run
/// let mut state = codetether_file_picker::FilePickerState::default();
/// codetether_file_picker::file_picker_page_up(&mut state);
/// ```
pub fn file_picker_page_up(state: &mut FilePickerState) {
    if state.mode == FilePickerMode::Viewer {
        state.preview_scroll = state.preview_scroll.saturating_sub(10);
    }
}

/// Scroll the full preview downward by ten lines within content bounds.
/// # Arguments
/// * `state` - Mutable browser/viewer state.
/// # Returns
/// Unit after applying the action if the current mode permits it.
/// # Examples
/// ```no_run
/// let mut state = codetether_file_picker::FilePickerState::default();
/// codetether_file_picker::file_picker_page_down(&mut state);
/// ```
pub fn file_picker_page_down(state: &mut FilePickerState) {
    if state.mode == FilePickerMode::Viewer {
        let max_scroll = state
            .preview
            .as_ref()
            .map(|preview| preview.lines.len().saturating_sub(1))
            .unwrap_or(0);
        state.preview_scroll = state.preview_scroll.saturating_add(10).min(max_scroll);
    }
}
