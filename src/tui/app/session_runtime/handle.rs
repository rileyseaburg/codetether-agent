//! Runtime command sender.
//!
//! This module exposes the lightweight handle used by the TUI to send commands
//! into the session runtime task. The handle hides the underlying channel and
//! provides intent-specific methods for prompt submission, cancellation, and
//! shutdown.

use tokio::sync::mpsc;

use super::{PromptRequest, SessionCommand};

/// Cloneable handle for controlling the TUI session runtime.
///
/// `TuiSessionHandle` is the caller-facing control surface for a background
/// session runtime. Clones share the same command channel, so UI widgets,
/// key handlers, and shutdown paths can all coordinate with the same runtime
/// without owning the runtime task itself.
///
/// The handle does not guarantee that a command is processed; successful sends
/// only mean the runtime channel accepted the command. If the runtime has
/// already stopped, submission returns the original prompt request so callers
/// can decide how to recover.
#[derive(Clone)]
pub(crate) struct TuiSessionHandle {
    tx: mpsc::Sender<SessionCommand>,
}

impl TuiSessionHandle {
    /// Wrap a runtime command channel.
    ///
    /// # Arguments
    ///
    /// * `tx` - Sender half of the session runtime command channel.
    ///
    /// # Returns
    ///
    /// A cloneable handle that forwards control requests to the runtime task.
    pub(super) fn new(tx: mpsc::Sender<SessionCommand>) -> Self {
        Self { tx }
    }

    /// Submit one moved-session prompt.
    ///
    /// The request is boxed into a [`SessionCommand::SubmitPrompt`] and sent to
    /// the runtime. The runtime may still reject the request later if it is busy;
    /// this method only reports whether the command channel accepted ownership
    /// of the request.
    ///
    /// # Arguments
    ///
    /// * `request` - Prompt request to transfer to the runtime task.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the command was queued. Returns the original
    /// [`PromptRequest`] when the runtime channel is closed before accepting it.
    ///
    /// # Errors
    ///
    /// The error value is the unsent prompt request, allowing the caller to keep
    /// or retry the prompt without reconstructing it.
    pub(crate) async fn submit(&self, request: PromptRequest) -> Result<(), PromptRequest> {
        let command = SessionCommand::SubmitPrompt(Box::new(request));
        match self.tx.send(command).await {
            Ok(()) => Ok(()),
            Err(err) => match err.0 {
                SessionCommand::SubmitPrompt(request) => Err(*request),
                SessionCommand::CancelCurrent | SessionCommand::Shutdown => unreachable!(),
            },
        }
    }

    /// Interrupt the active prompt, if any.
    ///
    /// Sends an asynchronous cancellation command and waits until the command is
    /// queued or the runtime channel is closed. A closed channel is ignored
    /// because cancellation is best-effort during teardown.
    pub(crate) async fn cancel_current(&self) {
        let _ = self.tx.send(SessionCommand::CancelCurrent).await;
    }

    /// Best-effort non-blocking interrupt for key handlers.
    ///
    /// This variant is intended for latency-sensitive UI input paths. It tries
    /// to enqueue a cancellation command immediately and silently drops the
    /// request if the channel is full or closed.
    pub(crate) fn request_cancel_current(&self) {
        let _ = self.tx.try_send(SessionCommand::CancelCurrent);
    }

    /// Ask the runtime to stop accepting work.
    ///
    /// Sends a shutdown command to the runtime task. The command is best-effort:
    /// if the runtime has already stopped and the channel is closed, the error
    /// is ignored.
    pub(crate) async fn shutdown(&self) {
        let _ = self.tx.send(SessionCommand::Shutdown).await;
    }
}
