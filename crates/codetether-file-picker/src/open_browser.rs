//! Build initial directory browser state and preview.
use crate::{
    preview_cache::refresh_preview,
    scan::scan_directory,
    types::{FilePickerMode, FilePickerState},
};
use std::path::Path;

/// Build a directory browser with its initial selected-entry preview.
/// # Arguments
/// * `cwd` - Initial directory and workspace root.
/// # Returns
/// Owned browser state after directory scanning.
/// # Examples
/// ```no_run
/// let state = codetether_file_picker::open_file_picker(std::path::Path::new("."));
/// assert!(state.active);
/// ```
pub fn open_file_picker(cwd: &Path) -> FilePickerState {
    let dir = cwd.to_path_buf();
    let entries = scan_directory(&dir);
    let mut file_picker = FilePickerState {
        active: true,
        workspace_dir: dir.clone(),
        dir: dir.clone(),
        all_entries: entries.clone(),
        entries,
        selected: 0,
        filter: String::new(),
        mode: FilePickerMode::Browse,
        preview: None,
        preview_scroll: 0,
    };
    refresh_preview(&mut file_picker);
    file_picker
}
