use crate::tui::app::state::App;
#[path = "tick/host.rs"]
mod host;

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
    codetether_tui_tick::run(&mut host::Native(app));
}

#[cfg(test)]
#[path = "tick_tests.rs"]
mod tests;
