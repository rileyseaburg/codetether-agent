//! Build a direct file-viewer state and preview.
use crate::{
    scan::scan_directory,
    types::{FilePickerMode, FilePickerState},
};
use std::path::Path;

/// Open a specific file directly in the read-only viewer, bypassing the
/// browser. Used by go-to-definition on a shared file.
/// Build a read-only file viewer with its surrounding directory listing.
/// # Arguments
/// * `workspace_dir` - Root used when the file has no parent.
/// * `file` - File whose preview should be opened.
/// # Returns
/// Owned viewer state with the requested preview.
/// # Examples
/// ```no_run
/// let state = codetether_file_picker::open_file_in_viewer(std::path::Path::new("."), std::path::Path::new("note.txt"));
/// assert!(state.active);
/// ```
pub fn open_file_in_viewer(workspace_dir: &Path, file: &Path) -> FilePickerState {
    let dir = file.parent().unwrap_or(workspace_dir).to_path_buf();
    let entries = scan_directory(&dir);
    let selected = entries.iter().position(|e| e.path == file).unwrap_or(0);
    FilePickerState {
        active: true,
        workspace_dir: workspace_dir.to_path_buf(),
        dir,
        all_entries: entries.clone(),
        entries,
        selected,
        filter: String::new(),
        mode: FilePickerMode::Viewer,
        preview: Some(super::preview::load_preview(file)),
        preview_scroll: 0,
    }
}
