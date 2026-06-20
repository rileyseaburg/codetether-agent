use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

#[path = "dirty.rs"]
mod dirty;
#[path = "tick_watchdog.rs"]
mod tick_watchdog;

pub(super) use tick_watchdog::check;

pub async fn run(app: &mut App) {
    let before = dirty::Snapshot::capture(app);
    super::autochat::drain_autochat(app);
    crate::tui::app::event_handlers::drain_voice_transcription(&mut app.state);
    if app.state.view_mode == crate::tui::models::ViewMode::Audit {
        crate::tui::audit_view::refresh_audit_snapshot(&mut app.state.audit).await;
    }
    let ralph_changed = app.state.ralph.drain_events();
    let swarm_changed = app.state.swarm.drain_events();
    let forage_changed = crate::tui::forage_run::drain_forage_updates(app);
    let shell_changed = crate::tui::app::input::shell_bg::drain_shell_events(app);
    app.state.needs_redraw |= ralph_changed
        || swarm_changed
        || forage_changed
        || shell_changed
        || before.changed_since(app);
}

pub fn before_draw(app: &mut App, worker_bridge: &Option<TuiWorkerBridge>) {
    let before = app.state.worker_bridge_registered_agents.len();
    crate::tui::app::worker_bridge::sync_worker_bridge_agents(app, worker_bridge);
    app.state.needs_redraw |= before != app.state.worker_bridge_registered_agents.len();
}
