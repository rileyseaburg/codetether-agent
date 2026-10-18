//! Project-memory recall command with application presentation bindings.
use std::path::Path;
/// Presentation operations used by the recall command.
pub trait View {
    /// Append the recall report to chat.
    fn push_system_message(&mut self, text: String);
    /// Scroll to the appended report.
    fn scroll_to_bottom(&mut self);
    /// Set the recall status line.
    fn set_status(&mut self, text: String);
}
/// Load project memories and present their recall report.
/// # Arguments
/// * `view` — Chat presentation bindings.
/// * `project_root` — Workspace containing the project memory file.
/// # Returns
/// Updates chat and status; unreadable or absent memory uses the empty report.
/// # Examples
/// ```no_run
/// # fn recall(view: &mut impl codetether_cognition_core::palace::command::View) {
/// codetether_cognition_core::palace::command::recall(view, std::path::Path::new("."));
/// # }
/// ```
pub fn recall(view: &mut impl View, project_root: &Path) {
    let beliefs = super::load_project_beliefs(project_root);
    view.push_system_message(super::recall_message(&beliefs));
    view.scroll_to_bottom();
    view.set_status("Project memory palace".to_string());
}
