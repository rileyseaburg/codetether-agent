//! Explicit propagation of parent controls through a spawned job's future.

use crate::ControlHandle;
tokio::task_local! { static CURRENT: ControlHandle; }

impl ControlHandle {
    /// Poll work with this handle installed in explicit task-local context.
    /// # Arguments
    /// `future` is the work whose nested calls should see this control.
    /// # Returns
    /// The future's output, restoring the prior context afterward.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example() {
    /// use codetether_session_control::ControlHandle;
    /// let handle = ControlHandle::new();
    /// handle.scope(async { assert!(ControlHandle::current().is_some()); }).await;
    /// # }
    /// ```
    pub async fn scope<F: std::future::Future>(&self, future: F) -> F::Output {
        CURRENT.scope(self.clone(), future).await
    }

    /// Read the handle installed in the current explicit task-local scope.
    /// # Returns
    /// A cloned handle, or `None` outside an installed scope.
    /// # Examples
    /// See [ControlHandle::scope] for installing and querying a scope.
    pub fn current() -> Option<Self> {
        CURRENT.try_with(Clone::clone).ok()
    }

    /// Spawn work while propagating the caller's explicit control context.
    /// # Arguments
    /// `future` is owned, sendable work for a Tokio task.
    /// # Returns
    /// A join handle yielding the future's output.
    /// # Panics
    /// Panics outside a Tokio runtime.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example() {
    /// let job = codetether_session_control::ControlHandle::spawn(async { 7 });
    /// assert_eq!(job.await.unwrap(), 7);
    /// # }
    /// ```
    pub fn spawn<F>(future: F) -> tokio::task::JoinHandle<F::Output>
    where
        F: std::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let control = Self::current();
        tokio::spawn(async move {
            match control {
                Some(control) => control.scope(future).await,
                None => future.await,
            }
        })
    }
}
