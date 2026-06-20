use std::path::Path;

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

use super::preview_cache::refresh_preview;
use super::scan::scan_directory;
use super::types::{FilePickerMode, FilePickerState};

pub fn open_file_picker(app: &mut App, cwd: &Path) {
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
    app.state.file_picker = file_picker;
    app.state.view_mode = ViewMode::FilePicker;
    app.state.status = "File browser: Enter opens, Alt+a attaches, Esc closes".to_string();
}

/// Open a specific file directly in the read-only viewer, bypassing the
/// browser. Used by go-to-definition on a shared file.
pub fn open_file_in_viewer(app: &mut App, workspace_dir: &Path, file: &Path) {
    let dir = file.parent().unwrap_or(workspace_dir).to_path_buf();
    let entries = scan_directory(&dir);
    let selected = entries.iter().position(|e| e.path == file).unwrap_or(0);
    app.state.file_picker = FilePickerState {
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
    };
    app.state.view_mode = ViewMode::FilePicker;
    app.state.status = format!("Viewing {}", file.display());
}
