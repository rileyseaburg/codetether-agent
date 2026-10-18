//! Document path handling and embedded buffer activation.
use super::View;
use std::path::Path;
/// Open an explicit command path or show the finder for an empty argument.
/// # Arguments
/// * `view` - Editor/finder state bindings.
/// * `cwd` - Working directory used for path resolution.
/// * `arg` - Command argument after whitespace/quote trimming.
/// # Returns
/// Completes after opening a document, reporting a read error or showing the finder.
/// # Examples
/// ```no_run
/// # fn example<V: codetether_editor::interactive::View>(view: &mut V) {
/// codetether_editor::interactive::open_argument(view, std::path::Path::new("."), "main.rs");
/// # }
/// ```
pub fn open_argument<V: View>(view: &mut V, cwd: &Path, arg: &str) {
    if arg.is_empty() {
        super::open_finder(view, cwd);
        return;
    }
    let path = cwd.join(arg);
    open_document(view, &path, arg);
}
/// Open a selected path with its workspace-relative display label.
/// # Arguments
/// * `view` - Editor state bindings.
/// * `cwd` - Workspace prefix removed from the displayed path when present.
/// * `path` - Selected file path.
/// # Returns
/// Completes after opening the buffer or reporting its read error.
/// # Examples
/// ```no_run
/// # fn example<V: codetether_editor::interactive::View>(view: &mut V) {
/// codetether_editor::interactive::open_path(view, std::path::Path::new("."), std::path::Path::new("main.rs"));
/// # }
/// ```
pub fn open_path<V: View>(view: &mut V, cwd: &Path, path: &Path) {
    let label = path.strip_prefix(cwd).unwrap_or(path).display().to_string();
    open_document(view, path, &label);
}
pub(super) fn open_document<V: View>(view: &mut V, path: &Path, label: &str) {
    match crate::FileBuffer::open(path) {
        Ok(buf) => {
            view.set_buffer(buf);
            view.reset_editor_scroll();
            view.show_editor();
            view.set_status(format!("Editing {label} — Ctrl+S save, Esc close"));
        }
        Err(e) => view.set_status(format!("Cannot open {label}: {e}")),
    }
}
