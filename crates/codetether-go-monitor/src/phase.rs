//! Serializable lifecycle of a background autonomous pipeline.

use serde::{Deserialize, Serialize};
/// Phase of a running go pipeline.
///
/// # Examples
/// ```
/// use codetether_go_monitor::GoRunPhase;
/// let phase = GoRunPhase::Starting;
/// match phase {
///     GoRunPhase::Starting => {},
///     GoRunPhase::Running => {},
///     GoRunPhase::Completed { .. } => {},
///     GoRunPhase::Failed { .. } => {},
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "phase", rename_all = "snake_case")]
pub enum GoRunPhase {
    /// The pipeline has been registered and is awaiting execution.
    Starting,
    /// The pipeline is executing.
    Running,
    /// Execution finished with story counts and a rendered summary.
    Completed {
        /// Number of successful stories.
        passed: usize,
        /// Total number of stories.
        total: usize,
        /// Whether every story passed.
        all_passed: bool,
        /// Branch containing the pipeline changes.
        feature_branch: String,
        /// Rendered pipeline result.
        summary: String,
    },
    /// Execution failed.
    Failed {
        /// Execution failure message.
        error: String,
    },
}
