//! Session load outcome reporting for `hydrate::complete`.

use crate::session::Session;

pub(super) enum SessionLoadOutcome {
    /// A prior session was successfully loaded and is now active.
    Loaded {
        msg_count: usize,
        title: Option<String>,
        /// Messages dropped by the tail-cap window (0 = full transcript loaded).
        dropped: usize,
        file_bytes: u64,
    },
    /// No prior session existed — a fresh session was started.
    Fresh,
    /// The session scan failed for an unexpected reason (timeout, corrupt
    /// file, etc.). A fresh session was started but the user should be told.
    ScanFailed { reason: String },
}

/// Build the outcome descriptor from the already-resolved session and any
/// scan-failure warning stored by `session_resolve`.
pub(super) fn from_session(session: &Session) -> SessionLoadOutcome {
    // If session_resolve stored a warning, surface it.
    if let Ok(reason) = std::env::var("CODETETHER_SESSION_LOAD_WARNING") {
        unsafe {
            std::env::remove_var("CODETETHER_SESSION_LOAD_WARNING");
        }
        return SessionLoadOutcome::ScanFailed { reason };
    }

    let msg_count = session.history().len();
    if msg_count == 0 && session.title.is_none() {
        // Heuristic: brand-new session has no messages and no title.
        SessionLoadOutcome::Fresh
    } else {
        SessionLoadOutcome::Loaded {
            msg_count,
            title: session.title.clone(),
            dropped: 0, // tail-cap drops are logged by session_resolve
            file_bytes: 0,
        }
    }
}
