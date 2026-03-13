use crate::session::list_sessions_for_directory;
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

pub async fn refresh_sessions(app: &mut App, cwd: &std::path::Path) {
    match list_sessions_for_directory(cwd).await {
        Ok(sessions) => {
            app.state.sessions = sessions;
            if app.state.selected_session >= app.state.filtered_sessions().len() {
                app.state.selected_session = app.state.filtered_sessions().len().saturating_sub(1);
            }
        }
        Err(err) => {
            app.state.status = format!("Failed to list sessions: {err}");
            app.state.sessions.clear();
            app.state.selected_session = 0;
        }
    }
}

pub fn return_to_chat(app: &mut App) {
    app.state.set_view_mode(ViewMode::Chat);
    app.state.status = "Back to chat".to_string();
}
