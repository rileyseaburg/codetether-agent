//! File picker nav operations.
use crate::FilePickerState;

use super::preview::load_preview;
use super::preview_cache::refresh_preview;
use super::scan::scan_directory;
use super::types::FilePickerMode;

/// Open the selected file preview or enter the selected directory.
/// # Arguments
/// * `state` - Mutable browser/viewer state.
/// * `_cwd` - Reserved workspace path; navigation uses the selected entry.
/// # Returns
/// Unit after applying the action if the current mode permits it.
/// # Examples
/// ```no_run
/// let mut state = codetether_file_picker::FilePickerState::default();
/// codetether_file_picker::file_picker_enter(&mut state, std::path::Path::new("."));
/// ```
pub fn file_picker_enter(state: &mut FilePickerState, _cwd: &std::path::Path) {
    let Some(entry) = state.entries.get(state.selected).cloned() else {
        return;
    };
    if entry.is_dir {
        state.dir = entry.path;
        let entries = scan_directory(&state.dir);
        state.all_entries = entries.clone();
        state.entries = entries;
        state.selected = 0;
        state.filter.clear();
        refresh_preview(state);
    } else {
        state.preview = Some(load_preview(&entry.path));
        state.preview_scroll = 0;
        state.mode = FilePickerMode::Viewer;
    }
}

/// Return from full preview to browsing, reporting whether it handled the action.
/// # Arguments
/// * `state` - Mutable browser/viewer state.
/// # Returns
/// True when viewer mode was closed; false when already browsing.
/// # Examples
/// ```no_run
/// let mut state = codetether_file_picker::FilePickerState::default();
/// codetether_file_picker::file_picker_escape(&mut state);
/// ```
pub fn file_picker_escape(state: &mut FilePickerState) -> bool {
    if state.mode == FilePickerMode::Viewer {
        state.mode = FilePickerMode::Browse;
        true
    } else {
        false
    }
}
