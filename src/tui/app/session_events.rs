//! TUI adapter for session runtime events.
//!
//! This module is the boundary between the session runtime and the terminal UI.
//! It receives [`SessionEvent`] values produced by the runtime, routes them
//! through the event-processing pipeline, updates UI retention state, and then
//! applies the resulting event to the visible application context.
//!
//! The adapter is intentionally thin: event-specific behavior lives in the
//! sibling modules below, while this module coordinates ordering and shared UI
//! side effects such as marking activity and keeping the chat view pinned to
//! the newest message when auto-follow is enabled.

mod context;
mod errors;
mod flow;
mod lifecycle;
mod pipeline;
mod retention;
mod text;
mod text_bound;
mod text_dispatch;
mod tools;
mod usage;

#[cfg(test)]
mod tests;

pub use usage::attach_usage_to_last_completion_message;

use crate::session::SessionEvent;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

/// Handles a single session runtime event and applies its effects to the TUI.
///
/// This function records that activity occurred, sends the event through the
/// session-event pipeline, trims retained UI data after successful processing,
/// and finally dispatches the processed event into the application context.
/// If the pipeline consumes the event and returns `None`, the current flow is
/// stopped instead of performing retention or context updates.
///
/// # Arguments
///
/// * `app` - Mutable TUI application state that receives timestamp, scrolling,
///   retention, flow, and context updates.
/// * `slot` - Runtime slot associated with the session that produced the event.
///   Pipeline stages may use this to update per-session runtime state.
/// * `worker_bridge` - Optional bridge used when event handling needs to
///   communicate with the background TUI worker.
/// * `evt` - Runtime event emitted by the session layer.
///
/// # Side Effects
///
/// Updates `app.state.main_last_event_at` to the current instant. When chat
/// auto-follow is enabled, the chat viewport is scrolled to the bottom before
/// pipeline processing. Depending on the pipeline result, this may also stop
/// the current flow, trim retained UI state, and update the displayed context.
///
/// # Errors
///
/// This function does not return errors. Pipeline failures are represented by
/// the pipeline's event output and flow-control behavior rather than by a
/// `Result`.
pub(crate) async fn handle_session_event(
    app: &mut App,
    slot: &mut SessionSlot,
    worker_bridge: &Option<TuiWorkerBridge>,
    evt: SessionEvent,
) {
    note_event(app);
    let Some(evt) = pipeline::run(app, slot, worker_bridge, evt).await else {
        return flow::stop(app);
    };
    retention::trim(app);
    context::handle_event(app, evt);
}

/// Records UI activity for an incoming session event.
///
/// The timestamp is used by the TUI to know when the main session last produced
/// activity. If chat auto-follow is enabled, the visible chat state is also
/// moved to the bottom so newly arriving output remains in view.
///
/// # Arguments
///
/// * `app` - Mutable application state whose activity timestamp and scroll
///   position should be updated.
///
/// # Side Effects
///
/// Mutates `app.state.main_last_event_at` and may mutate the chat scroll state.
fn note_event(app: &mut App) {
    app.state.main_last_event_at = Some(std::time::Instant::now());
    if app.state.chat_auto_follow {
        app.state.scroll_to_bottom();
    }
}
