use super::{Control, SessionHandle};
use crate::request::SessionCommand;
impl<Q, C: Control> SessionHandle<Q, C> {
    /// Interrupt the active prompt, if any.
    ///
    /// Sends an asynchronous cancellation command and waits until the command is
    /// queued or the runtime channel is closed. A closed channel is ignored
    /// because cancellation is best-effort during teardown.
    pub async fn cancel_current(&self) {
        if !self.active_cancel.notify() {
            let _ = self.tx.send(SessionCommand::CancelCurrent).await;
        }
    }

    /// Best-effort non-blocking interrupt for key handlers.
    ///
    /// This variant is intended for latency-sensitive UI input paths. It signals
    /// the active executor directly, falling back to the command queue only
    /// during the short prompt-submission handoff.
    pub fn request_cancel_current(&self) {
        if !self.active_cancel.notify() {
            let _ = self.tx.try_send(SessionCommand::CancelCurrent);
        }
    }

    /// Ask the runtime to stop accepting work.
    ///
    /// Sends a shutdown command to the runtime task. The command is best-effort:
    /// if the runtime has already stopped and the channel is closed, the error
    /// is ignored.
    pub async fn shutdown(&self) {
        let _ = self.tx.send(SessionCommand::Shutdown).await;
    }
}
