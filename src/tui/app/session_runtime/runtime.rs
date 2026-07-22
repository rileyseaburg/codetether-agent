//! Session runtime event loop.
//!
//! This module owns the asynchronous command loop that sits between TUI input
//! handlers and prompt execution. It receives [`SessionCommand`] values through
//! a [`TuiSessionHandle`], forwards streamed [`SessionEvent`] values to the TUI,
//! and emits [`SessionNotice`] messages when session ownership changes.
//!
//! The runtime shares only a focused cancellation signal with its handle so key
//! handlers can interrupt immediately without waiting on the command queue.

use tokio::sync::mpsc;

use crate::session::SessionEvent;

use super::{SessionCommand, SessionNotice, TuiSessionHandle, active_cancel::ActiveCancel};

/// Spawn the TUI-owned session runtime task.
///
/// The returned [`TuiSessionHandle`] sends commands to the spawned task. The
/// task continues until it receives a shutdown command or its command channel is
/// closed.
///
/// # Arguments
///
/// * `event_tx` - Channel used by prompt execution to stream [`SessionEvent`]
///   values back to the TUI.
/// * `notice_tx` - Channel used by the runtime to report [`SessionNotice`]
///   lifecycle updates such as prompt start, completion, or failure.
///
/// # Returns
///
/// A handle connected to the runtime command channel.
///
/// # Side Effects
///
/// Spawns a Tokio task and allocates an internal command channel with capacity
/// for eight queued commands.
pub(crate) fn spawn(
    event_tx: mpsc::Sender<SessionEvent>,
    notice_tx: mpsc::Sender<SessionNotice>,
) -> TuiSessionHandle {
    let (cmd_tx, cmd_rx) = mpsc::channel(8);
    let active_cancel = ActiveCancel::default();
    tokio::spawn(run(cmd_rx, event_tx, notice_tx, active_cancel.clone()));
    TuiSessionHandle::new(cmd_tx, active_cancel)
}

/// Run the session runtime command loop until shutdown or channel closure.
///
/// The loop waits for commands from [`TuiSessionHandle`]. Prompt executors clear
/// their shared active ownership before publishing their completion notice.
///
/// # Arguments
///
/// * `cmd_rx` - Receiver for runtime commands.
/// * `event_tx` - Sender used by prompt execution for streamed session events.
/// * `notice_tx` - Sender used for runtime lifecycle notices.
/// * `active_cancel` - Shared direct signal for the current prompt.
///
/// # Side Effects
///
/// Handles prompt submission, cancellation, and shutdown commands. The function
/// awaits until a command requests termination, all command senders are dropped,
/// or the internal select loop otherwise reaches channel closure.
async fn run(
    mut cmd_rx: mpsc::Receiver<SessionCommand>,
    event_tx: mpsc::Sender<SessionEvent>,
    notice_tx: mpsc::Sender<SessionNotice>,
    active_cancel: ActiveCancel,
) {
    while let Some(command) = cmd_rx.recv().await {
        if super::loop_step::handle(command, &active_cancel, &event_tx, &notice_tx).await {
            break;
        }
    }
}
