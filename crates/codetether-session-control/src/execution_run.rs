//! Run a future under its executor lease and delegated task-local control.
use crate::RunGuard;
impl RunGuard {
    /// Run an executor until it completes or shared control requests cancellation.
    ///
    /// # Errors
    /// Returns cancellation or the executor's own error without claiming completion.
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
        let handle = self.handle();
        tokio::select! {
            biased;
            _ = self.execution.cancelled() => anyhow::bail!("Session execution cancelled"),
            result = handle.scope(future) => result,
        }
    }
}
