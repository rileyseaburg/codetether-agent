//! Worker task-claim request contract.
use serde::Deserialize;
/// Identify a pending task to claim atomically.
/// # Examples
/// ```
/// let request = codetether_task_api::worker_claim::ClaimRequest { task_id: "task-1".into() };
/// assert_eq!(request.task_id, "task-1");
/// ```

#[derive(Debug, Deserialize)]
pub struct ClaimRequest {
    /// Identifier of the pending task to transition to processing.
    pub task_id: String,
}
