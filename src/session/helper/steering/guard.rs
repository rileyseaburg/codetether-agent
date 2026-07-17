//! Prompt-run lifetime guard for steering inbox cleanup.

/// Keeps a session inbox open only for the lifetime of one prompt run.
pub(crate) struct RunGuard(String);

impl RunGuard {
    /// Open the session inbox and return its cleanup guard.
    pub(crate) fn open(session_id: &str) -> Self {
        super::open(session_id);
        Self(session_id.to_string())
    }
}

impl Drop for RunGuard {
    fn drop(&mut self) {
        super::clear(&self.0);
    }
}
