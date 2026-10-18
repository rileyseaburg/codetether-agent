//! Frontend visible-state counters for tick and event selection.
/// Declare the frontend's visible-state snapshot projection.
///
/// # Examples
/// ```text
/// codetether_tui_tick::declare_frontend_snapshot!(App);
/// ```
#[macro_export]
macro_rules! declare_frontend_snapshot {
    ($app:path $(,)?) => {
        use $app as App;
        use $crate::{Snapshot, animation_epoch};
        pub(super) fn capture(app: &App) -> Snapshot {
            Snapshot {
                messages: app.state.messages.len(),
                input: app.state.composer.input.len(),
                status: app.state.status.len(),
                streaming: app.state.streaming.streaming_text.len(),
                bus: app.state.bus_log.entries.len(),
                audit_tick: app.state.audit.refresh_counter,
                queue: app.state.worker.worker_task_queue.len(),
                processing: app.state.processing,
                animation_epoch: animation_epoch(
                    app.state.processing,
                    app.state.interlude.is_some(),
                ),
            }
        }
    };
}
