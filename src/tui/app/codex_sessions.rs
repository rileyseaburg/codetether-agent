use crate::session::{Session, import_codex_sessions_for_directory};
use crate::tui::app::message_text::sync_messages_from_source;
use crate::tui::app::session_fork::fork_for_app;
use crate::tui::app::session_load_status::load_status_with_original;
use crate::tui::app::session_loader::load_session_for_tui;
use crate::tui::app::session_sync::{refresh_sessions, return_to_chat};
use crate::tui::app::state::App;
use std::path::Path;

pub async fn load_selected_session(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    session_id: &str,
) {
    match load_session_for_tui(session_id).await {
        Ok(loaded) => {
            let dropped = loaded.dropped;
            let file_bytes = loaded.file_bytes;
            *session = loaded.session;
            let Some(original_id) = fork_for_app(app, session, dropped).await else {
                return;
            };
            session.attach_global_bus_if_missing();
            crate::tool::agent::persistence::hydrate_best_effort(&session.id).await;
            app.state.auto_apply_edits = session.metadata.auto_apply_edits;
            app.state.use_worktree = session.metadata.use_worktree;
            app.state.session_id = Some(session.id.clone());
            let source_id = original_id.as_deref().unwrap_or(&session.id).to_string();
            sync_messages_from_source(app, session, &source_id, dropped > 0);
            refresh_sessions(app, cwd).await;
            app.state.clear_session_filter();
            return_to_chat(app);
            app.state.status =
                load_status_with_original(session, dropped, file_bytes, original_id.as_deref());
        }
        Err(error) => app.state.status = format!("Failed to load session: {error}"),
    }
}

pub async fn import_workspace_sessions(app: &mut App, cwd: &Path) {
    match import_codex_sessions_for_directory(cwd).await {
        Ok(report) => {
            refresh_sessions(app, cwd).await;
            app.state.status = format!(
                "Imported {} Codex sessions, skipped {}",
                report.imported, report.skipped
            );
        }
        Err(error) => app.state.status = format!("Codex import failed: {error}"),
    }
}
