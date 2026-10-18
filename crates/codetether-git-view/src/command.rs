//! Working-tree report presentation for the diff utility command.
use std::path::Path;
/// Native composer and transcript operations for a diff report.
/// # Implementors
/// Frontends supply existing input/status/message updates.
pub trait View {
    /// Clear consumed input.
    fn clear_input(&mut self);
    /// Replace visible status.
    fn set_status(&mut self, status: String);
    /// Append the report as a system message.
    fn push_system(&mut self, text: String);
    /// Follow the report in the transcript.
    fn scroll_to_bottom(&mut self);
}
/// Build and display the existing working-tree report.
/// # Arguments
/// Receives native presentation operations and the repository directory.
/// # Returns
/// Reports clean, changed or unavailable Git state, then follows the message.
/// # Examples
/// ```no_run
/// fn diff<V: codetether_git_view::command::View>(view: &mut V) {
///     codetether_git_view::command::show(view, std::path::Path::new("."));
/// }
/// ```
pub fn show(host: &mut impl View, cwd: &Path) {
    finish(host, crate::working_tree_report(cwd));
}
fn finish(host: &mut impl View, text: String) {
    host.clear_input();
    host.set_status("Diff shown".to_string());
    host.push_system(text);
    host.scroll_to_bottom();
}
