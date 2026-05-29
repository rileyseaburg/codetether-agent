use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub async fn run(app: &mut App) {
    super::autochat::drain_autochat(app);
    crate::tui::app::event_handlers::drain_voice_transcription(&mut app.state);
    if app.state.view_mode == crate::tui::models::ViewMode::Audit {
        crate::tui::audit_view::refresh_audit_snapshot(&mut app.state.audit).await;
    }
    app.state.ralph.drain_events();
}

pub fn before_draw(app: &mut App, worker_bridge: &Option<TuiWorkerBridge>) {
    crate::tui::app::worker_bridge::sync_worker_bridge_agents(app, worker_bridge);
}
