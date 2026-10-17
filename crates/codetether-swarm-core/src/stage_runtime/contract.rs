use super::*;
/// Runtime operations used by the shared stage coordinator.
///
/// # Implementors
/// Application adapters retain backend resources, cancellation, event delivery,
/// publication and coordinator updates while the shared loop controls ordering.
#[async_trait]
pub trait StageRuntime: Send {
    /// Return the original plan used to determine the highest stage.
    fn subtasks(&self) -> &[SubTask];
    /// Wait for readiness; return false to stop stage traversal gracefully.
    async fn may_continue(&mut self) -> bool;
    /// Read tasks and failure metadata for a single stage.
    fn stage_input(&self, stage: usize) -> StageInput<'_>;
    /// Report failures caused by unsuccessful prerequisites before execution.
    fn report_blocked(&self, tasks: &[SubTask], results: &[SubTaskResult]);
    /// Execute runnable tasks using the selected backend.
    ///
    /// # Errors
    /// Returns a backend error, stopping traversal before result recording.
    async fn execute_tasks(&mut self, tasks: Vec<SubTask>) -> Result<Vec<SubTaskResult>>;
    /// Publish results, record elapsed statistics and retain results in that order.
    async fn record_results(&mut self, stage: usize, started: Instant, results: Vec<SubTaskResult>);
}
