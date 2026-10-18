//! Native visible-state projection for shared tick redraw policy.
use crate::tui::app::state::App;
use codetether_tui_tick::{Snapshot, animation_epoch};
pub(super) fn capture(app: &App) -> Snapshot {
    Snapshot {
        messages: app.state.messages.len(),
        input: app.state.input.len(),
        status: app.state.status.len(),
        streaming: app.state.streaming.streaming_text.len(),
        bus: app.state.bus_log.entries.len(),
        audit_tick: app.state.audit.refresh_counter,
        queue: app.state.worker.worker_task_queue.len(),
        processing: app.state.processing,
        animation_epoch: animation_epoch(app.state.processing, app.state.interlude.is_some()),
    }
}
