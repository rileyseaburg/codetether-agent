use crate::session::{Session, import_codex_sessions_for_directory, load_or_import_session};
use crate::tui::app::message_text::sync_messages_from_session;
use crate::tui::app::session_sync::{refresh_sessions, return_to_chat};
use crate::tui::app::state::App;
use std::path::Path;

pub async fn load_selected_session(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    session_id: &str,
) {
    match load_or_import_session(session_id).await {
        Ok(loaded) => {
            *session = loaded;
            session.attach_global_bus_if_missing();
            app.state.auto_apply_edits = session.metadata.auto_apply_edits;
            app.state.use_worktree = session.metadata.use_worktree;
            app.state.session_id = Some(session.id.clone());
            sync_messages_from_session(app, session);
            refresh_sessions(app, cwd).await;
            app.state.clear_session_filter();
            return_to_chat(app);
            app.state.status = format!(
                "Loaded session {}",
                session.title.clone().unwrap_or_else(|| session.id.clone())
            );
        }
        Err(error) => {
            app.state.status = format!("Failed to load session: {error}");
        }
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
        Err(error) => {
            app.state.status = format!("Codex import failed: {error}");
        }
    }
}
