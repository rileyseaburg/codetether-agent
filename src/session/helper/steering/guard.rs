//! Prompt-run lifetime guard for steering inbox cleanup.

/// Keeps a session inbox open only for the lifetime of one prompt run.
pub(crate) struct RunGuard {
    session_id: String,
    #[cfg(unix)]
    _endpoint: Option<super::ipc::Endpoint>,
}

impl RunGuard {
    /// Open the session inbox and return its cleanup guard.
    pub(crate) fn open(session_id: &str) -> Self {
        super::open(session_id);
        #[cfg(unix)]
        let endpoint = super::ipc::Endpoint::start(session_id)
            .inspect_err(
                |error| tracing::warn!(%error, %session_id, "Session steering unavailable"),
            )
            .ok();
        Self {
            session_id: session_id.to_string(),
            #[cfg(unix)]
            _endpoint: endpoint,
        }
    }
}

impl Drop for RunGuard {
    fn drop(&mut self) {
        super::clear(&self.session_id);
    }
}
