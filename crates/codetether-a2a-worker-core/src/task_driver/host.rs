//! Application effects required by the shared claimed-task lifecycle.
use crate::{
    claim::ClaimedTaskData,
    outcome::{TaskInner, TaskOutcome},
    task_timeline::TaskTimeline,
};
use anyhow::Result;
use async_trait::async_trait;
use codetether_provenance::ClaimProvenance;
/// Application operations for task progress, claiming, execution, and release.
///
/// # Implementors
/// Hosts retain runtime state and transports. The driver invokes these
/// operations in order and applies the claim-adjusted execution timeout.
#[async_trait]
pub trait TaskHandlerHost: Sync {
    /// Synchronize task progress to the active worker runtime.
    async fn sync_timeline(&self, timeline: &TaskTimeline);
    /// Claim a task and update its timeline with the server's accepted budget.
    /// # Errors
    /// Returns signing, transport, or claim decoding errors.
    async fn claim(
        &self,
        task_id: &str,
        timeline: &mut TaskTimeline,
    ) -> Result<Option<ClaimedTaskData>>;
    /// Execute a claimed task using the accepted attribution and provider keys.
    /// # Errors
    /// Returns the task execution error for outcome normalization.
    async fn execute(
        &self,
        task: &serde_json::Value,
        task_id: &str,
        title: &str,
        provenance: &ClaimProvenance,
        provider_keys: Option<serde_json::Value>,
        timeline: &mut TaskTimeline,
    ) -> TaskInner;
    /// Release an execution outcome and finish progress cleanup.
    /// # Errors
    /// Returns release signing, transport, or server rejection errors.
    async fn release(
        &self,
        task_id: &str,
        outcome: TaskOutcome,
        timeline: &mut TaskTimeline,
    ) -> Result<()>;
}
