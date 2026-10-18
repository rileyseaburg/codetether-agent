//! Shared-file actions: show git diff, and open in the file viewer.

use std::path::Path;

use crate::Host;
use codetether_git_view::capture_file_diff;

use super::resolve::latest_shared_file;

/// Show the git diff for the most recently shared file in the `/git` webview.
///
/// # Arguments
/// * `host` — Frontend state and file operations.
/// * `workspace_dir` — Base directory for transcript references.
/// # Returns
/// Updates the host view, or its status if no file is available.
/// # Examples
/// ```rust,no_run
/// fn act(host: &mut impl codetether_chat_files::Host) {
///     codetether_chat_files::show_shared_file_diff(host, std::path::Path::new("."));
/// }
/// ```
pub fn show_shared_file_diff(host: &mut impl Host, workspace_dir: &Path) {
    let Some(file) = latest_shared_file(host.messages(), workspace_dir) else {
        host.set_status("No shared file to diff.".to_string());
        return;
    };
    host.set_diff(capture_file_diff(workspace_dir, &file));
    host.set_captured_at(chrono::Local::now().format("%H:%M:%S").to_string());
    host.save_scroll_state();
    host.show_git();
    host.set_status(format!("Diff: {}", file.display()));
}

/// Open the most recently shared file in the standard TUI file viewer.
///
/// # Arguments
/// * `host` — Frontend state and file operations.
/// * `workspace_dir` — Base directory for transcript references.
/// # Returns
/// Updates the host view, or its status if no file is available.
/// # Examples
/// ```rust,no_run
/// fn act(host: &mut impl codetether_chat_files::Host) {
///     codetether_chat_files::goto_shared_file_definition(host, std::path::Path::new("."));
/// }
/// ```
pub fn goto_shared_file_definition(host: &mut impl Host, workspace_dir: &Path) {
    let Some(file) = latest_shared_file(host.messages(), workspace_dir) else {
        host.set_status("No shared file to open.".to_string());
        return;
    };
    if !file.is_file() {
        host.set_status(format!("File not found: {}", file.display()));
        return;
    }
    host.save_scroll_state();
    host.open_viewer(workspace_dir, &file);
}
