//! Workspace archive import and picker refresh.
use super::{Host, View};
use std::path::Path;
/// Import workspace sessions, refreshing the picker only on success.
///
/// # Arguments
/// * `app` — UI receiving catalog refresh and status updates.
/// * `cwd` — Workspace directory whose archive sessions should be imported.
///
/// # Returns
/// Updates the picker on success or shows the import failure in status.
///
/// # Examples
/// ```no_run
/// # async fn import<H: codetether_session_resume::interactive::Host>(
/// #     view: &mut impl codetether_session_resume::interactive::View<H::Session>) {
/// codetether_session_resume::interactive::import_workspace_sessions::<H>(
///     view, std::path::Path::new("."),
/// ).await;
/// # }
/// ```
pub async fn import_workspace_sessions<H: Host>(app: &mut impl View<H::Session>, cwd: &Path) {
    match H::import_workspace(cwd).await {
        Ok((imported, skipped)) => {
            app.refresh(cwd).await;
            app.set_status(format!(
                "Imported {} Codex sessions, skipped {}",
                imported, skipped
            ));
        }
        Err(error) => app.set_status(format!("Codex import failed: {error}")),
    }
}
