//! File picker nav jump operations.
use crate::FilePickerState;

use super::preview_cache::refresh_preview;
use super::types::FilePickerMode;

/// Select the first browser entry and refresh its preview.
/// # Arguments
/// * `state` - Mutable browser/viewer state.
/// # Returns
/// Unit after applying the action if the current mode permits it.
/// # Examples
/// ```no_run
/// let mut state = codetether_file_picker::FilePickerState::default();
/// codetether_file_picker::file_picker_select_first(&mut state);
/// ```
pub fn file_picker_select_first(state: &mut FilePickerState) {
    let picker = state;
    if picker.mode == FilePickerMode::Browse && !picker.entries.is_empty() {
        picker.selected = 0;
        refresh_preview(picker);
    }
}

/// Select the last browser entry and refresh its preview.
/// # Arguments
/// * `state` - Mutable browser/viewer state.
/// # Returns
/// Unit after applying the action if the current mode permits it.
/// # Examples
/// ```no_run
/// let mut state = codetether_file_picker::FilePickerState::default();
/// codetether_file_picker::file_picker_select_last(&mut state);
/// ```
pub fn file_picker_select_last(state: &mut FilePickerState) {
    let picker = state;
    if picker.mode == FilePickerMode::Browse && !picker.entries.is_empty() {
        picker.selected = picker.entries.len() - 1;
        refresh_preview(picker);
    }
}
