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
    spinner_frame: u8,
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
            spinner_frame: spinner_frame(app.state.processing),
        }
    }

    pub(super) fn changed_since(&self, app: &App) -> bool {
        self != &Self::capture(app)
    }
}

/// Spinner frame index that advances only while `processing`.
///
/// While idle this returns a constant `0`, so the snapshot is stable and
/// no redraw is triggered (0% idle CPU). While a turn is in flight it
/// cycles every 100 ms so the spinner animates without a free-running
/// render loop.
fn spinner_frame(processing: bool) -> u8 {
    if !processing {
        return 0;
    }
    let ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    ((ms / 100) % 10) as u8
}
