//! Bounded draining for queued TUI session events.
//!
//! This module keeps live stream rendering responsive by consuming a limited
//! batch of already-queued [`SessionEvent`] values per UI pass. It also offers
//! a completion-notice drain that flushes pending stream events before the TUI
//! applies session ownership changes from the runtime.
//!
//! The two drain modes share the same implementation in [`core`], but use
//! different limits and completion handling:
//!
//! - [`drain_batch`] performs a small, non-blocking UI pass drain so rendering
//!   stays responsive while a session is actively streaming.
//! - [`drain_before_notice`] performs a larger flush before a completion notice
//!   is handled, reducing the chance that final stream output is displayed
//!   after ownership or runtime state has already changed.

use tokio::sync::mpsc;

use crate::session::SessionEvent;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

mod core;

/// Maximum number of queued session events processed during a regular UI pass.
///
/// This limit prevents a busy session stream from monopolizing the event loop.
/// Any events left in the channel remain queued for a later drain pass.
const DRAIN_BATCH_LIMIT: usize = 64;

/// Maximum number of queued session events flushed before handling a notice.
///
/// Completion notices can transfer or finalize session state, so this larger
/// limit gives pending stream output a chance to reach the UI before the notice
/// changes runtime ownership or presentation state.
const NOTICE_DRAIN_LIMIT: usize = 1024;

/// Drains a bounded batch of pending [`SessionEvent`] values into the TUI state.
///
/// This function is intended for normal render-loop processing. It consumes up
/// to [`DRAIN_BATCH_LIMIT`] events that are already available on `event_rx` and
/// applies their effects to `app`, `slot`, and, when present, `worker_bridge`.
/// The drain is deliberately bounded so a high-volume stream cannot block
/// redraws or input handling for too long.
///
/// # Parameters
///
/// * `app` - Mutable TUI application state that receives rendered session
///   updates and status changes.
/// * `slot` - Mutable runtime slot for the session whose events are being
///   drained.
/// * `worker_bridge` - Optional bridge used when drained events must update or
///   coordinate with the worker-facing TUI integration.
/// * `event_rx` - Receiver containing queued session events from the active
///   session runtime.
///
/// # Side Effects
///
/// Updates in-memory TUI and session runtime state. Events consumed from
/// `event_rx` are removed from the channel.
///
/// # Errors
///
/// This function does not return errors. Channel closure and per-event handling
/// are managed by the shared drain implementation.
pub(crate) async fn drain_batch(
    app: &mut App,
    slot: &mut SessionSlot,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_rx: &mut mpsc::Receiver<SessionEvent>,
) {
    core::drain(app, slot, worker_bridge, event_rx, DRAIN_BATCH_LIMIT, false).await;
}

/// Flushes pending session events before a completion or ownership notice.
///
/// This drain mode is used immediately before the TUI handles a runtime notice
/// that may finalize the session, transfer ownership, or otherwise change how
/// the active session is represented. It consumes up to [`NOTICE_DRAIN_LIMIT`]
/// queued events and enables notice-aware handling in the shared drain core so
/// pending stream output is applied before the notice takes effect.
///
/// # Parameters
///
/// * `app` - Mutable TUI application state that receives final stream and
///   status updates.
/// * `slot` - Mutable runtime slot associated with the session being finalized
///   or updated.
/// * `worker_bridge` - Optional bridge used to propagate event effects to the
///   worker-facing TUI integration.
/// * `event_rx` - Receiver containing queued session events that should be
///   flushed before the notice is processed.
///
/// # Side Effects
///
/// Mutates TUI/session runtime state and removes consumed events from
/// `event_rx`. Because this mode uses a larger limit than [`drain_batch`], it
/// may process more queued work in a single call.
///
/// # Errors
///
/// This function does not return errors. Channel closure and event-specific
/// failures are handled internally by the shared drain implementation.
pub(crate) async fn drain_before_notice(
    app: &mut App,
    slot: &mut SessionSlot,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_rx: &mut mpsc::Receiver<SessionEvent>,
) {
    core::drain(app, slot, worker_bridge, event_rx, NOTICE_DRAIN_LIMIT, true).await;
}

#[cfg(test)]
mod tests;
