//! Reuse an explicit parent run during a same-session executor handoff.

use super::{ControlHandle, RunGuard, steering};

/// An exclusive entrypoint owner or a borrow from its explicit task-local scope.
pub(crate) enum ExecutionScope {
    Owned(RunGuard),
    Borrowed(ControlHandle),
}

impl ExecutionScope {
    /// Acquire ownership before entering a loop; independent duplicates fail.
    /// # Errors
    /// Returns an error if another task owns this session.
    pub(crate) fn open(session_id: &str) -> anyhow::Result<Self> {
        match ControlHandle::current().filter(|handle| handle.session_id() == Some(session_id)) {
            Some(handle) => Ok(Self::Borrowed(handle)),
            None => RunGuard::open(session_id).map(Self::Owned),
        }
    }

    /// Run under the acquired lease, preserving parent cancellation and holds.
    /// # Errors
    /// Returns cancellation, durable-hold, or executor errors.
    pub(crate) async fn run<T>(
        &mut self,
        future: impl std::future::Future<Output = anyhow::Result<T>>,
    ) -> anyhow::Result<T> {
        match self {
            Self::Owned(guard) => guard.run(future).await,
            Self::Borrowed(control) => {
                control.ready().await?;
                steering::open(control.session_id().expect("attached session handle"));
                tokio::select! {
                    biased;
                    _ = control.cancelled() => anyhow::bail!("Session execution cancelled"),
                    result = control.scope(future) => result,
                }
            }
        }
    }
}
