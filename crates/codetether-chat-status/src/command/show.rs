//! Refresh, format and present the status report.
use super::{View, lines, policy::policy_line};
use std::path::Path;
/// Show session, policy, UI settings, context and completion status.
/// # Arguments
/// Receives the native view, workspace path and report heading.
/// # Returns
/// Publishes one report and clears the consumed input.
/// # Examples
/// ```no_run
/// async fn status<V: codetether_chat_status::command::View>(view: &mut V) {
///     codetether_chat_status::command::show(view, std::path::Path::new("."), "Status").await;
/// }
/// ```
pub async fn show(host: &mut impl View, cwd: &Path, label: &str) {
    let policy = policy_line(cwd).await;
    let state = host.snapshot();
    let context = lines::context(&state);
    let model = state.model.unwrap_or("not selected");
    let last = lines::completion(&state);
    let text = format!(
        "{label}\nSession: {}\nWorkspace: {}\nModel: {model}\n{policy}\n{}\n{context}\n{last}",
        state.session_id,
        cwd.display(),
        lines::tui(&state),
    );
    host.push_system(text);
    host.scroll_to_bottom();
    host.clear_input();
    host.set_status(format!("{label} shown"));
}
