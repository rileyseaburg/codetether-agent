//! Workspace finder initialization and selected-file confirmation.
use super::View;
use codetether_fuzzy_finder::{FuzzyFindState, scan_workspace, take_selected_path};
use std::path::Path;
/// Scan the workspace and show the fuzzy finder.
/// # Arguments
/// * `view` - Finder slot and status output.
/// * `cwd` - Workspace root to scan.
/// # Returns
/// Completes after installing the candidate list and status message.
/// # Examples
/// ```no_run
/// # fn example<V: codetether_editor::interactive::View>(view: &mut V) {
/// codetether_editor::interactive::open_finder(view, std::path::Path::new("."));
/// # }
/// ```
pub fn open_finder<V: View>(view: &mut V, cwd: &Path) {
    let candidates = scan_workspace(cwd);
    let count = candidates.len();
    *view.finder() = Some(FuzzyFindState::new(candidates));
    view.set_status(format!(
        "Fuzzy open: {count} files — type to filter, Enter opens, Esc cancels"
    ));
}
/// Open the selected finder path when one is available.
/// # Arguments
/// * `view` - Finder and editor state.
/// * `cwd` - Workspace used for the editor status label.
/// # Returns
/// Completes without opening when no path is selected.
/// # Examples
/// ```no_run
/// # fn example<V: codetether_editor::interactive::View>(view: &mut V) {
/// codetether_editor::interactive::confirm_finder(view, std::path::Path::new("."));
/// # }
/// ```
pub fn confirm_finder<V: View>(view: &mut V, cwd: &Path) {
    let chosen = take_selected_path(view.finder());
    let Some(path) = chosen else { return };
    super::open_path(view, cwd, &path);
}
