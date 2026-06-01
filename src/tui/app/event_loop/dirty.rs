//! Dirty-state helpers for the TUI event loop.

use crate::tui::app::state::App;

#[derive(PartialEq, Eq)]
pub(super) struct Snapshot {
    messages: usize,
    input: usize,
    status: usize,
    streaming: usize,
    bus: usize,
    audit_tick: u64,
    queue: usize,
    processing: bool,
}

impl Snapshot {
    pub(super) fn capture(app: &App) -> Self {
        Self {
            messages: app.state.messages.len(),
            input: app.state.input.len(),
            status: app.state.status.len(),
            streaming: app.state.streaming_text.len(),
            bus: app.state.bus_log.entries.len(),
            audit_tick: app.state.audit.refresh_counter,
            queue: app.state.worker_task_queue.len(),
            processing: app.state.processing,
        }
    }

    pub(super) fn changed_since(&self, app: &App) -> bool {
        self != &Self::capture(app)
    }
}
