//! sessions command execution bindings.
use super::*;
pub(super) async fn run(host: &mut Dispatch<'_>, kind: SimpleCommand) {
    let app = &mut *host.app;
    let cwd = host.cwd;
    let session = &mut *host.session;
    let registry = host.registry;
    match kind {
        SimpleCommand::Sessions => {
            refresh_sessions(app, cwd).await;
            app.state.clear_session_filter();
            app.state.set_view_mode(ViewMode::Sessions);
            app.state.status = "Session picker".to_string();
        }
        SimpleCommand::ImportCodex => {
            codex_sessions::import_workspace_sessions(app, cwd).await;
        }
        SimpleCommand::Model => open_model_picker(app, session, registry).await,
        SimpleCommand::New => {
            crate::tui::app::session_sync::create_new_session(app, cwd, session).await
        }
        _ => unreachable!("command routed to the wrong native adapter"),
    }
}
