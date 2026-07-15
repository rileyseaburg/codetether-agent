//! Session load outcome descriptor for `hydrate::complete`.
//!
//! Produced directly by [`super::session_resolve`] — never inferred from
//! session state after the fact.

pub(super) enum SessionLoadOutcome {
    /// A prior session was successfully loaded and is now active.
    Loaded {
        msg_count: usize,
        title: Option<String>,
        /// Messages dropped by the tail-cap window (0 = full transcript loaded).
        dropped: usize,
        /// Original full-history session preserved when continuation is forked.
        source_id: Option<String>,
    },
    /// No prior session existed — a fresh session was started.
    Fresh,
    /// The session scan failed for an unexpected reason (timeout, corrupt
    /// file, etc.). A fresh session was started but the user should be told.
    ScanFailed { reason: String },
}

impl SessionLoadOutcome {
    pub(super) fn history_source<'a>(&'a self, current: &'a str) -> (&'a str, bool) {
        match self {
            Self::Loaded {
                dropped, source_id, ..
            } => (source_id.as_deref().unwrap_or(current), *dropped > 0),
            Self::Fresh | Self::ScanFailed { .. } => (current, false),
        }
    }
}
