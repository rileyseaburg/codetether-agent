//! Application view and status updates around shared picker constructors.
use super::View;
use std::path::Path;
/// Apply the file-picker workflow through frontend state bindings.
/// # Arguments
/// Receives the frontend view and relevant workspace/file path, when required.
/// # Returns
/// Updates picker state, view selection and attachment presentation.
pub fn open_file_picker(view: &mut impl View, cwd: &Path) {
    *view.picker() = crate::open_file_picker(cwd);
    view.show_picker();
    view.set_status("File browser: Enter opens, Alt+a attaches, Esc closes".to_string());
}
/// Apply the file-picker workflow through frontend state bindings.
/// # Arguments
/// Receives the frontend view and relevant workspace/file path, when required.
/// # Returns
/// Updates picker state, view selection and attachment presentation.
pub fn open_file_in_viewer(view: &mut impl View, workspace_dir: &Path, file: &Path) {
    *view.picker() = crate::open_file_in_viewer(workspace_dir, file);
    view.show_picker();
    view.set_status(format!("Viewing {}", file.display()));
}
