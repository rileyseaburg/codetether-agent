//! Dirty-state helpers for the TUI event loop.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
    animation_epoch: u64,
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
            animation_epoch: animation_epoch(app.state.processing),
        }
    }

    pub(super) fn changed_since(&self, app: &App) -> bool {
        self != &Self::capture(app)
    }
}

/// Animation epoch that advances only while `processing`.
///
/// While idle this remains `0`. During processing it advances once per second,
/// avoiding expensive full-chat redraws solely for decorative animation.
fn animation_epoch(processing: bool) -> u64 {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    animation_epoch_at(processing, elapsed)
}

pub(super) fn animation_epoch_at(processing: bool, elapsed: Duration) -> u64 {
    if processing { elapsed.as_secs() } else { 0 }
}

#[cfg(test)]
#[path = "dirty_tests.rs"]
mod tests;
