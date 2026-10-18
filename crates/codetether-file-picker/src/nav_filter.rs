//! File picker nav filter operations.
use crate::FilePickerState;

use super::filter::rescan_with_filter;
use super::types::FilePickerMode;

/// Remove one filter character and reapply the cached directory filter.
/// # Arguments
/// * `state` - Mutable browser/viewer state.
/// # Returns
/// Unit after applying the action if the current mode permits it.
/// # Examples
/// ```no_run
/// let mut state = codetether_file_picker::FilePickerState::default();
/// codetether_file_picker::file_picker_filter_backspace(&mut state);
/// ```
pub fn file_picker_filter_backspace(state: &mut FilePickerState) {
    if state.mode == FilePickerMode::Browse {
        state.filter.pop();
        rescan_with_filter(state);
    }
}

/// Append a filter character and reapply the cached directory filter.
/// # Arguments
/// * `state` - Mutable browser/viewer state.
/// * `c` - Character to append.
/// # Returns
/// Unit after applying the action if the current mode permits it.
/// # Examples
/// ```no_run
/// let mut state = codetether_file_picker::FilePickerState::default();
/// codetether_file_picker::file_picker_filter_push(&mut state, 'a');
/// ```
pub fn file_picker_filter_push(state: &mut FilePickerState, c: char) {
    if state.mode == FilePickerMode::Browse {
        state.filter.push(c);
        rescan_with_filter(state);
    }
}
