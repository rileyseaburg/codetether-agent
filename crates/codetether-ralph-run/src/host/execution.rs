//! Native checkout and scheduling callbacks.
/// Execute the selected run mode with the existing native policy and resources.
/// # Implementors
/// Bind the existing checkout, parallel and sequential entry points.
#[async_trait::async_trait]
pub trait Execution: Send + Sync {
    /// Checkout the configured feature branch before running stories.
    /// # Errors
    /// Returns the native checkout or policy error.
    fn git_checkout(&self, branch: &str) -> anyhow::Result<()>;
    /// Execute the parallel scheduler.
    /// # Errors
    /// Propagates scheduler and persistence errors.
    async fn run_parallel(&mut self) -> anyhow::Result<()>;
    /// Execute the sequential scheduler.
    /// # Errors
    /// Propagates scheduler and persistence errors.
    async fn run_sequential(&mut self) -> anyhow::Result<()>;
}
