//! Reuse an explicit parent run during a same-session executor handoff.

use crate::{ControlHandle, RunGuard};
use codetether_session_steering as steering;

/// An exclusive entrypoint owner or a borrow from its explicit task-local scope.
/// # Examples
/// ```rust,no_run
/// use codetether_session_control::{ExecutionScope, GateFuture};
/// fn check(_: &str) -> GateFuture<'_> { Box::pin(async { Ok(()) }) }
/// # async fn example() -> anyhow::Result<()> {
/// let mut scope = ExecutionScope::open("session-1234", check)?;
/// assert_eq!(scope.run(async { Ok(7) }).await?, 7);
/// # Ok(()) }
/// ```
pub enum ExecutionScope {
    /// Independent entry that owns the exclusive executor guard.
    Owned(RunGuard),
    /// Explicit task-local handoff retaining the existing parent owner.
    Borrowed(ControlHandle),
}

impl ExecutionScope {
    /// Acquire ownership before entering a loop; independent duplicates fail.
    /// # Errors
    /// Returns an error if another task owns this session.
    /// # Arguments
    /// `session_id` identifies the requested executor; `gate` is retained only
    /// when this call acquires a new owner. Borrowed scopes retain their parent's gate.
    /// # Returns
    /// An owning scope or a same-session scope borrowed from explicit task context.
    /// # Examples
    /// See [ExecutionScope] for opening and executing a scope.
    pub fn open(session_id: &str, gate: crate::Gate) -> anyhow::Result<Self> {
        match ControlHandle::current().filter(|handle| handle.session_id() == Some(session_id)) {
            Some(handle) => Ok(Self::Borrowed(handle)),
            None => RunGuard::open(session_id, gate).map(Self::Owned),
        }
    }

    /// Run under the acquired lease, preserving parent cancellation and holds.
    /// # Errors
    /// Returns cancellation, durable-hold, or executor errors.
    /// # Arguments
    /// `future` is the executor work to run under this scope's control.
    /// # Returns
    /// The future's result unless cancellation or a durable gate rejects execution.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example(scope: &mut codetether_session_control::ExecutionScope) -> anyhow::Result<()> {
    /// assert_eq!(scope.run(async { Ok(7) }).await?, 7);
    /// # Ok(()) }
    /// ```
    pub async fn run<T>(
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
