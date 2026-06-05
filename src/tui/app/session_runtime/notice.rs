//! Notices emitted by the TUI session runtime.
//!
//! The session runtime sends these notices to the TUI event loop to describe
//! progress through a prompt execution lifecycle. Each notice represents a
//! state transition that the UI can render or use to update the active
//! [`Session`].

use crate::session::Session;

/// Runtime state updates consumed by the event loop.
///
/// A notice is emitted when prompt execution starts, finishes successfully, or
/// fails. Variants that contain a [`Session`] transfer ownership of the updated
/// session back to the event loop so it can persist or display the latest state.
pub(crate) enum SessionNotice {
    /// A prompt execution task has started.
    ///
    /// This variant carries no session data because the event loop already owns
    /// or can reference the session state that was submitted for execution.
    Started,
    /// A prompt execution task completed successfully.
    ///
    /// Contains the session after the prompt run so callers can resume normal
    /// interaction with the updated conversation state.
    Finished(Session),
    /// A prompt execution task failed or panicked.
    ///
    /// The `session` field contains the session associated with the failed run.
    /// The `error` field contains a displayable failure message suitable for UI
    /// status text or diagnostics.
    Failed { session: Session, error: String },
}
