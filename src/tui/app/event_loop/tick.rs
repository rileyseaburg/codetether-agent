use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

#[path = "dirty.rs"]
mod dirty;
#[path = "tick_watchdog.rs"]
mod tick_watchdog;

pub(super) use tick_watchdog::check;

pub async fn refresh_audit(app: &mut App) {
    if app.state.view_mode == crate::tui::models::ViewMode::Audit {
        crate::tui::audit_view::refresh_audit_snapshot(&mut app.state.audit).await;
    }
}

pub async fn run(app: &mut App) {
    let before = dirty::Snapshot::capture(app);
    super::autochat::drain_autochat(app);
    crate::tui::app::event_handlers::drain_voice_transcription(&mut app.state);
    let ralph_changed = app.state.ralph.drain_events();
    let swarm_changed = crate::tool::swarm_execute::tui_bridge::drain(&mut app.state.swarm);
    let forage_changed = crate::tui::forage_run::drain_forage_updates(app);
    let shell_changed = crate::tui::app::input::shell_bg::drain_shell_events(app);
    let history_changed = crate::tui::app::state::history_page::drain(app);
    app.state.needs_redraw |= ralph_changed
        || swarm_changed
        || forage_changed
        || shell_changed
        || history_changed
        || before.changed_since(app);
}

pub fn before_draw(
    app: &mut App,
    worker_bridge: &Option<TuiWorkerBridge>,
    synced_cursor: &mut Option<u64>,
) {
    if !should_sync(*synced_cursor, app.state.bus_cursor) {
        return;
    }
    *synced_cursor = Some(app.state.bus_cursor);
    crate::tui::app::worker_bridge::sync_worker_bridge_agents(app, worker_bridge);
}

fn should_sync(synced: Option<u64>, current: u64) -> bool {
    synced != Some(current)
}

#[cfg(test)]
#[path = "tick_tests.rs"]
mod tests;
