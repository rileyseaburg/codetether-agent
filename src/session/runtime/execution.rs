//! Shared executor lifetime, cancellation, and steering-resource cleanup.

/// Owns one session execution lease and its steering transport until completion.
pub(crate) struct RunGuard {
    execution: codetether_session_runtime::Execution,
    #[cfg(unix)]
    _endpoint: Option<super::steering::Endpoint>,
}

impl RunGuard {
    /// Attach child orchestration to this execution's existing ownership lease.
    pub(crate) fn handle(&self) -> super::ControlHandle {
        super::ControlHandle::attached(self.execution.lease().clone())
    }

    /// Open the session inbox and return its cleanup guard.
    ///
    /// # Errors
    /// Returns an error if another executor already owns the session.
    pub(crate) fn open(session_id: &str) -> anyhow::Result<Self> {
        let execution = crate::session::runtime::CONTROL
            .begin(session_id)
            .ok_or_else(|| anyhow::anyhow!("Session already has an active executor"))?;
        super::steering::open(session_id);
        #[cfg(unix)]
        let endpoint = super::steering::Endpoint::start(session_id)
            .inspect_err(
                |error| tracing::warn!(%error, %session_id, "Session steering unavailable"),
            )
            .ok();
        Ok(Self {
            execution,
            #[cfg(unix)]
            _endpoint: endpoint,
        })
    }

    /// Run an executor until it completes or shared control requests cancellation.
    ///
    /// # Errors
    /// Returns cancellation or the executor's own error without claiming completion.
    pub(crate) async fn run<T>(
        &mut self,
        future: impl std::future::Future<Output = anyhow::Result<T>>,
    ) -> anyhow::Result<T> {
        let handle = self.handle();
        tokio::select! {
            biased;
            _ = self.execution.cancelled() => anyhow::bail!("Session execution cancelled"),
            result = handle.scope(future) => result,
        }
    }
}

impl Drop for RunGuard {
    fn drop(&mut self) {
        let lease = self.execution.lease().clone();
        crate::session::runtime::CONTROL.finish_with(&lease, || {
            #[cfg(unix)]
            self._endpoint.take();
            super::steering::clear(lease.session_id());
        });
    }
}
