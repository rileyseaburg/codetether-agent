//! Single command handling step for the session runtime.
//!
//! This module translates one [`SessionCommand`] into the concrete runtime action
//! needed by the session worker. Prompt submissions are delegated to the submit
//! path, while cancellation and shutdown commands notify the active executor via
//! its shared [`Notify`] handle.

use tokio::sync::mpsc;

use crate::session::SessionEvent;

use super::{SessionCommand, SessionNotice, active_cancel::ActiveCancel};

/// Handles one command received by the session runtime loop.
///
/// The runtime loop calls this for each incoming command. Submitting a prompt
/// may spawn asynchronous prompt execution through `super::loop_submit::submit`.
/// Cancellation requests notify the current in-flight executor, if one exists.
/// Shutdown requests also notify the executor and ask the outer loop to stop.
///
/// # Arguments
///
/// * `command` - Command to apply to the current session runtime state.
/// * `cancel` - Shared cancellation signal for the active prompt execution.
/// * `event_tx` - Channel used by prompt execution to publish streamed
///   [`SessionEvent`] values.
/// * `notice_tx` - Channel used to publish lifecycle notices for the session
///   runtime.
/// * `done_tx` - Channel used by prompt execution to signal completion to the
///   runtime loop.
///
/// # Returns
///
/// Returns `true` when the caller should break out of the runtime loop. This is
/// only requested for [`SessionCommand::Shutdown`]. Other commands return
/// `false`.
///
/// # Side Effects
///
/// May send lifecycle notices, spawn prompt execution, or wake the active
/// cancellation notifier depending on the command.
pub(super) async fn handle(
    command: SessionCommand,
    cancel: &ActiveCancel,
    event_tx: &mpsc::Sender<SessionEvent>,
    notice_tx: &mpsc::Sender<SessionNotice>,
    done_tx: &mpsc::Sender<()>,
) -> bool {
    match command {
        SessionCommand::SubmitPrompt(request) => {
            super::loop_submit::submit(*request, cancel, event_tx, notice_tx, done_tx).await
        }
        SessionCommand::CancelCurrent => notify_cancel(cancel, false),
        SessionCommand::Shutdown => notify_cancel(cancel, true),
    }
}

/// Notifies the active executor that cancellation was requested.
///
/// The notifier is optional because cancellation and shutdown are valid even
/// when no prompt is currently running. In that case this helper has no wake-up
/// side effect and simply returns the loop-control value supplied by the caller.
///
/// # Arguments
///
/// * `cancel` - Cancellation handle shared with the in-flight executor.
/// * `should_break` - Whether the runtime loop should stop after the
///   notification attempt.
///
/// # Returns
///
/// Returns `should_break` unchanged so callers can combine cancellation
/// notification with command-loop control.
fn notify_cancel(cancel: &ActiveCancel, should_break: bool) -> bool {
    cancel.notify();
    should_break
}
