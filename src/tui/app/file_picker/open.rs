use std::path::Path;

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

use super::preview_cache::refresh_preview;
use super::scan::scan_directory;
use super::types::{FilePickerMode, FilePickerState};

pub fn open_file_picker(app: &mut App, cwd: &Path) {
    let dir = cwd.to_path_buf();
    let mut file_picker = FilePickerState {
        active: true,
        workspace_dir: dir.clone(),
        dir: dir.clone(),
        entries: scan_directory(&dir),
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
