use crate::tui::app::state::App;

#[path = "tick/approval.rs"]
mod approval;
#[path = "dirty.rs"]
mod dirty;
#[path = "tick/retry.rs"]
mod retry;
#[path = "tick_watchdog.rs"]
mod tick_watchdog;
#[path = "tick/worker_sync.rs"]
mod worker_sync;

pub(super) use worker_sync::before_draw;
#[cfg(test)]
use worker_sync::should_sync;
pub(super) use {retry::check_and_retry, tick_watchdog::check};

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
    let symbols_changed = crate::tui::app::symbols::drain_refresh(app);
    let approval_changed = approval::reconcile(app);
    app.state.needs_redraw |= ralph_changed
        || swarm_changed
        || forage_changed
        || shell_changed
        || history_changed
        || symbols_changed
        || approval_changed
        || before.changed_since(app);
}

#[cfg(test)]
#[path = "tick_tests.rs"]
mod tests;
