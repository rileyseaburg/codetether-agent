//! Bind shared file-picker opening to native view state.
use crate::tui::app::state::App;
use std::path::Path;
pub fn open_file_picker(app: &mut App, cwd: &Path) {
    codetether_file_picker::interactive::open_file_picker(&mut super::host::Native(app), cwd);
}
pub fn open_file_in_viewer(app: &mut App, workspace_dir: &Path, file: &Path) {
    codetether_file_picker::interactive::open_file_in_viewer(
        &mut super::host::Native(app),
        workspace_dir,
        file,
    );
}
