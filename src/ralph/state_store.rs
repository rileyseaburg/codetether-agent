//! Ralph State Store - trait and types for persistent run state
//!
//! Abstracts Ralph run state persistence so it can be backed by
//! in-memory storage (tests/local), HTTP API (dashboard integration),
//! or any future backend.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::types::{Prd, ProgressEntry, RalphConfig, RalphStatus};

/// Persistent state for a single Ralph run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RalphRunState {
    pub run_id: String,
    pub okr_id: Option<String>,
    pub prd: Prd,
    pub config: RalphConfig,
    pub status: RalphStatus,
    pub current_iteration: usize,
    pub max_iterations: usize,
    #[serde(default)]
    pub progress_log: Vec<ProgressEntry>,
    #[serde(default)]
    pub story_results: Vec<StoryResultEntry>,
    pub error: Option<String>,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

/// Result of a single story execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryResultEntry {
    pub story_id: String,
    pub title: String,
    pub passed: bool,
    pub iteration: usize,
    pub error: Option<String>,
}

/// Lightweight summary for listing runs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RalphRunSummary {
    pub run_id: String,
    pub okr_id: Option<String>,
    pub status: RalphStatus,
    pub passed: usize,
    pub total: usize,
    pub current_iteration: usize,
    pub created_at: String,
}

/// Abstraction over Ralph run state persistence.
///
/// All methods are fire-and-forget from the caller's perspective —
/// implementations should log errors internally rather than failing
/// the Ralph pipeline.
#[async_trait]
pub trait RalphStateStore: Send + Sync {
    /// Create a new run record
    async fn create_run(&self, state: &RalphRunState) -> anyhow::Result<()>;

    /// Update the run status
    async fn update_status(&self, run_id: &str, status: RalphStatus) -> anyhow::Result<()>;

    /// Record iteration progress
    async fn update_iteration(&self, run_id: &str, iteration: usize) -> anyhow::Result<()>;

    /// Record a story result
    async fn record_story_result(
        &self,
        run_id: &str,
        result: &StoryResultEntry,
    ) -> anyhow::Result<()>;

    /// Append a progress log entry
    async fn append_progress(&self, run_id: &str, entry: &ProgressEntry) -> anyhow::Result<()>;

    /// Update the PRD (e.g. after marking stories passed)
    async fn update_prd(&self, run_id: &str, prd: &Prd) -> anyhow::Result<()>;

    /// Set error message
    async fn set_error(&self, run_id: &str, error: &str) -> anyhow::Result<()>;

    /// Mark run as completed
    async fn complete_run(&self, run_id: &str, status: RalphStatus) -> anyhow::Result<()>;

    /// Get full run state
    async fn get_run(&self, run_id: &str) -> anyhow::Result<Option<RalphRunState>>;

    /// List all runs (summary only)
    async fn list_runs(&self) -> anyhow::Result<Vec<RalphRunSummary>>;
}
