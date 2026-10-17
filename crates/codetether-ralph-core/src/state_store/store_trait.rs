//! Asynchronous persistence boundary for Ralph run state.
use super::*;

/// Abstraction over Ralph run state persistence.
///
/// All methods are fire-and-forget from the caller's perspective —
/// implementations should log errors internally rather than failing
/// the Ralph pipeline.
///
/// # Implementors
/// Backends implement asynchronous persistence and retrieval for one run.
#[async_trait]
pub trait RalphStateStore: Send + Sync {
    /// Create a new run record
    ///
    /// # Arguments
    /// * `state` - Run record to create.
    /// # Returns
    /// Unit after the backend processes the update.
    /// # Errors
    /// Returns backend-specific persistence or transport errors.
    /// # Examples
    /// ```no_run
    /// # use codetether_ralph_core::{Prd, ProgressEntry, RalphStatus};
    /// # use codetether_ralph_core::state_store::{RalphStateStore, RalphRunState, StoryResultEntry};
    /// # async fn example(store: &dyn RalphStateStore, state: &RalphRunState) -> anyhow::Result<()> {
    /// store.create_run(state).await?;
    /// # Ok(()) }
    /// ```
    async fn create_run(&self, state: &RalphRunState) -> anyhow::Result<()>;

    /// Update the run status
    ///
    /// # Arguments
    /// * `run_id, status` - Run identifier and replacement status.
    /// # Returns
    /// Unit after the backend processes the update.
    /// # Errors
    /// Returns backend-specific persistence or transport errors.
    /// # Examples
    /// ```no_run
    /// # use codetether_ralph_core::{Prd, ProgressEntry, RalphStatus};
    /// # use codetether_ralph_core::state_store::{RalphStateStore, RalphRunState, StoryResultEntry};
    /// # async fn example(store: &dyn RalphStateStore) -> anyhow::Result<()> {
    /// store.update_status("run-1", RalphStatus::Running).await?;
    /// # Ok(()) }
    /// ```
    async fn update_status(&self, run_id: &str, status: RalphStatus) -> anyhow::Result<()>;

    /// Record iteration progress
    ///
    /// # Arguments
    /// * `run_id, iteration` - Run identifier and current iteration.
    /// # Returns
    /// Unit after the backend processes the update.
    /// # Errors
    /// Returns backend-specific persistence or transport errors.
    /// # Examples
    /// ```no_run
    /// # use codetether_ralph_core::{Prd, ProgressEntry, RalphStatus};
    /// # use codetether_ralph_core::state_store::{RalphStateStore, RalphRunState, StoryResultEntry};
    /// # async fn example(store: &dyn RalphStateStore) -> anyhow::Result<()> {
    /// store.update_iteration("run-1", 1).await?;
    /// # Ok(()) }
    /// ```
    async fn update_iteration(&self, run_id: &str, iteration: usize) -> anyhow::Result<()>;

    /// Record a story result
    ///
    /// # Arguments
    /// * `run_id, result` - Run identifier and story result to record.
    /// # Returns
    /// Unit after the backend processes the update.
    /// # Errors
    /// Returns backend-specific persistence or transport errors.
    /// # Examples
    /// ```no_run
    /// # use codetether_ralph_core::{Prd, ProgressEntry, RalphStatus};
    /// # use codetether_ralph_core::state_store::{RalphStateStore, RalphRunState, StoryResultEntry};
    /// # async fn example(store: &dyn RalphStateStore, result: &StoryResultEntry) -> anyhow::Result<()> {
    /// store.record_story_result("run-1", result).await?;
    /// # Ok(()) }
    /// ```
    async fn record_story_result(
        &self,
        run_id: &str,
        result: &StoryResultEntry,
    ) -> anyhow::Result<()>;

    /// Append a progress log entry
    ///
    /// # Arguments
    /// * `run_id, entry` - Run identifier and progress entry to append.
    /// # Returns
    /// Unit after the backend processes the update.
    /// # Errors
    /// Returns backend-specific persistence or transport errors.
    /// # Examples
    /// ```no_run
    /// # use codetether_ralph_core::{Prd, ProgressEntry, RalphStatus};
    /// # use codetether_ralph_core::state_store::{RalphStateStore, RalphRunState, StoryResultEntry};
    /// # async fn example(store: &dyn RalphStateStore, entry: &ProgressEntry) -> anyhow::Result<()> {
    /// store.append_progress("run-1", entry).await?;
    /// # Ok(()) }
    /// ```
    async fn append_progress(&self, run_id: &str, entry: &ProgressEntry) -> anyhow::Result<()>;

    /// Update the PRD (e.g. after marking stories passed)
    ///
    /// # Arguments
    /// * `run_id, prd` - Run identifier and replacement PRD.
    /// # Returns
    /// Unit after the backend processes the update.
    /// # Errors
    /// Returns backend-specific persistence or transport errors.
    /// # Examples
    /// ```no_run
    /// # use codetether_ralph_core::{Prd, ProgressEntry, RalphStatus};
    /// # use codetether_ralph_core::state_store::{RalphStateStore, RalphRunState, StoryResultEntry};
    /// # async fn example(store: &dyn RalphStateStore, prd: &Prd) -> anyhow::Result<()> {
    /// store.update_prd("run-1", prd).await?;
    /// # Ok(()) }
    /// ```
    async fn update_prd(&self, run_id: &str, prd: &Prd) -> anyhow::Result<()>;

    /// Set error message
    ///
    /// # Arguments
    /// * `run_id, error` - Run identifier and failure message.
    /// # Returns
    /// Unit after the backend processes the update.
    /// # Errors
    /// Returns backend-specific persistence or transport errors.
    /// # Examples
    /// ```no_run
    /// # use codetether_ralph_core::{Prd, ProgressEntry, RalphStatus};
    /// # use codetether_ralph_core::state_store::{RalphStateStore, RalphRunState, StoryResultEntry};
    /// # async fn example(store: &dyn RalphStateStore) -> anyhow::Result<()> {
    /// store.set_error("run-1", "failed").await?;
    /// # Ok(()) }
    /// ```
    async fn set_error(&self, run_id: &str, error: &str) -> anyhow::Result<()>;

    /// Mark run as completed
    ///
    /// # Arguments
    /// * `run_id, status` - Run identifier and terminal status.
    /// # Returns
    /// Unit after the backend processes the update.
    /// # Errors
    /// Returns backend-specific persistence or transport errors.
    /// # Examples
    /// ```no_run
    /// # use codetether_ralph_core::{Prd, ProgressEntry, RalphStatus};
    /// # use codetether_ralph_core::state_store::{RalphStateStore, RalphRunState, StoryResultEntry};
    /// # async fn example(store: &dyn RalphStateStore) -> anyhow::Result<()> {
    /// store.complete_run("run-1", RalphStatus::Completed).await?;
    /// # Ok(()) }
    /// ```
    async fn complete_run(&self, run_id: &str, status: RalphStatus) -> anyhow::Result<()>;

    /// Get full run state
    ///
    /// # Arguments
    /// * `run_id` - Run identifier to retrieve.
    /// # Returns
    /// Optional stored run.
    /// # Errors
    /// Returns backend-specific persistence or transport errors.
    /// # Examples
    /// ```no_run
    /// # use codetether_ralph_core::{Prd, ProgressEntry, RalphStatus};
    /// # use codetether_ralph_core::state_store::{RalphStateStore, RalphRunState, StoryResultEntry};
    /// # async fn example(store: &dyn RalphStateStore) -> anyhow::Result<()> {
    /// let _state = store.get_run("run-1").await?;
    /// # Ok(()) }
    /// ```
    async fn get_run(&self, run_id: &str) -> anyhow::Result<Option<RalphRunState>>;

    /// List all runs (summary only)
    ///
    /// # Returns
    /// Available run summaries.
    /// # Errors
    /// Returns backend-specific persistence or transport errors.
    /// # Examples
    /// ```no_run
    /// # use codetether_ralph_core::{Prd, ProgressEntry, RalphStatus};
    /// # use codetether_ralph_core::state_store::{RalphStateStore, RalphRunState, StoryResultEntry};
    /// # async fn example(store: &dyn RalphStateStore) -> anyhow::Result<()> {
    /// let _runs = store.list_runs().await?;
    /// # Ok(()) }
    /// ```
    async fn list_runs(&self) -> anyhow::Result<Vec<RalphRunSummary>>;
}
