//! File validation, bounded snippet creation and composer installation.
use super::{View, composer, notice};
use crate::{build_file_share_snippet, display_path_for_workspace};
use std::path::Path;
/// Attach a selected file to the existing composer.
/// # Arguments
/// * `view` - Composer and notice bindings.
/// * `workspace_dir` - Workspace used to resolve and label the path.
/// * `path` - Selected absolute or relative file path.
/// * `max_bytes` - File content limit passed to the snippet builder.
/// # Returns
/// Completes after a notice; invalid or unreadable paths leave input unchanged.
/// # Examples
/// ```no_run
/// # fn example<V: codetether_file_share::interactive::View>(view: &mut V) {
/// codetether_file_share::interactive::attach(view, std::path::Path::new("."), std::path::Path::new("README.md"), 65536);
/// # }
/// ```
pub fn attach<V: View>(view: &mut V, workspace_dir: &Path, path: &Path, max_bytes: usize) {
    let file_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        workspace_dir.join(path)
    };
    if !file_path.exists() {
        view.notice(
            format!("File not found: {}", file_path.display()),
            "File not found",
        );
        return;
    }
    if !file_path.is_file() {
        view.notice(
            format!("Not a file: {}", file_path.display()),
            "Path is not a file",
        );
        return;
    }
    let display_path = display_path_for_workspace(&file_path, workspace_dir);
    match build_file_share_snippet(&file_path, &display_path, max_bytes) {
        Ok((snippet, truncated, binary)) => {
            composer::append(view, &snippet);
            let message = notice::success(&display_path, truncated, binary, max_bytes);
            view.notice(message, "Attached file to composer");
        }
        Err(err) => view.notice(
            format!("Failed to attach file {}: {err}", file_path.display()),
            "Failed to attach file",
        ),
    }
}
