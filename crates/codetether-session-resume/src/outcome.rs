//! Explicit outcome of resolving a session for startup.
//! The host constructs this result while loading or starting a session.

/// Result of selecting or creating the startup session.
///
/// # Examples
/// ```
/// let outcome = codetether_session_resume::SessionLoadOutcome::Fresh;
/// assert!(outcome.status().starts_with("New session"));
/// ```
pub enum SessionLoadOutcome {
    /// A prior session was successfully loaded and is now active.
    Loaded {
        /// Number of messages loaded into the visible session.
        msg_count: usize,
        /// Display label captured while resolving the session.
        label: String,
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
    /// Select the durable history source and whether older messages exist.
    ///
    /// # Arguments
    /// * `current` — Current session ID when no original source was retained.
    ///
    /// # Returns
    /// Source ID and whether the loaded outcome was truncated.
    ///
    /// # Examples
    /// ```
    /// let outcome = codetether_session_resume::SessionLoadOutcome::Fresh;
    /// assert_eq!(outcome.history_source("current"), ("current", false));
    /// ```
    pub fn history_source<'a>(&'a self, current: &'a str) -> (&'a str, bool) {
        match self {
            Self::Loaded {
                dropped, source_id, ..
            } => (source_id.as_deref().unwrap_or(current), *dropped > 0),
            Self::Fresh | Self::ScanFailed { .. } => (current, false),
        }
    }
}
