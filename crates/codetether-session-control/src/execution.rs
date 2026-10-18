//! Acquire a session executor and its steering transport.
//! Shared executor lifetime, cancellation, and steering-resource cleanup.

/// Owns one session execution lease and its steering transport until completion.
/// # Examples
/// ```rust,no_run
/// use codetether_session_control::{GateFuture, RunGuard};
/// fn check(_: &str) -> GateFuture<'_> { Box::pin(async { Ok(()) }) }
/// # async fn example() -> anyhow::Result<()> {
/// let guard = RunGuard::open("session-1234", check)?;
/// let control = guard.handle();
/// assert!(!control.is_cancelled());
/// # Ok(()) }
/// ```
pub struct RunGuard {
    pub(crate) execution: codetether_session_runtime::Execution,
    #[cfg(unix)]
    pub(crate) _endpoint: Option<codetether_session_steering::Endpoint>,
    pub(crate) gate: crate::Gate,
}
impl RunGuard {
    /// Attach child orchestration to this execution's existing ownership lease.
    /// # Examples
    /// See [RunGuard] for obtaining a handle without transferring ownership.
    pub fn handle(&self) -> crate::ControlHandle {
        crate::ControlHandle::attached(self.execution.lease().clone(), self.gate)
    }

    /// Open the session inbox and return its cleanup guard.
    ///
    /// # Errors
    /// Returns an error if another executor already owns the session.
    /// # Arguments
    /// `session_id` identifies the executor; `gate` checks durable state for
    /// attached handles before their provider or tool steps.
    /// # Returns
    /// The exclusive executor guard, with a best-effort steering endpoint.
    /// # Panics
    /// Panics if runtime ownership synchronization is poisoned, or Unix socket
    /// startup is attempted outside a Tokio runtime.
    /// # Examples
    /// See [RunGuard] for construction inside an async executor.
    pub fn open(session_id: &str, gate: crate::Gate) -> anyhow::Result<Self> {
        let execution = crate::CONTROL
            .begin(session_id)
            .ok_or_else(|| anyhow::anyhow!("Session already has an active executor"))?;
        codetether_session_steering::open(session_id);
        #[cfg(unix)]
        let endpoint = codetether_session_steering::Endpoint::start(session_id)
            .inspect_err(
                |error| tracing::warn!(target: "codetether_agent::session::runtime::execution", %error, %session_id, "Session steering unavailable"),
            )
            .ok();
        Ok(Self {
            execution,
            gate,
            #[cfg(unix)]
            _endpoint: endpoint,
        })
    }
}
