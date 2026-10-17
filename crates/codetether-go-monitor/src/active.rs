//! Shared run metadata and process-wide registry for launchers and monitors.

use crate::GoRunPhase;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
/// Metadata for an active go run.
///
/// # Examples
/// ```
/// use codetether_go_monitor::{ActiveGoRun, GoRunPhase};
/// let run = ActiveGoRun {
///     okr_id: "okr-id".into(), task: "Implement a feature".into(),
///     model: "model".into(), started_at: "timestamp".into(),
///     working_dir: ".".into(), prd_filename: "prd.json".into(),
///     progress_filename: "progress.txt".into(), phase: GoRunPhase::Starting,
/// };
/// assert_eq!(run.okr_id, "okr-id");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveGoRun {
    /// Identifier of the owning objective.
    pub okr_id: String,
    /// Original task description.
    pub task: String,
    /// Resolved model name.
    pub model: String,
    /// UTC start timestamp in RFC 3339 format.
    pub started_at: String,
    /// Working directory recorded at launch.
    pub working_dir: String,
    /// Path of the generated PRD file.
    pub prd_filename: String,
    /// Path of the pipeline progress notes.
    pub progress_filename: String,
    /// Current execution phase.
    pub phase: GoRunPhase,
}
/// Global registry of active (and recently completed) go runs.
pub static ACTIVE_GO_RUNS: std::sync::LazyLock<Mutex<HashMap<String, ActiveGoRun>>> =
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));
