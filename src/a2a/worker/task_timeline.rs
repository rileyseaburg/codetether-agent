//! Task pipeline instrumentation for debug tracing and timeout diagnosis.

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Instant;

/// Named checkpoints along the task processing pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskCheckpoint {
    TaskReceived,
    SlotReserved,
    ClaimRequested,
    Claimed,
    MetadataParsed,
    WorkspaceReady,
    SessionReady,
    GitHookInstalled,
    ProvidersLoaded,
    ModelSelected,
    AgentStarting,
    AgentRunning,
    AgentDone,
    SessionSaved,
    CommitStaging,
    CommitCreated,
    CommitPushing,
    CommitPushed,
    PrCreating,
    PrCreated,
    Releasing,
    Released,
    Completed,
    GracefulShutdown,
    Failed,
}

impl TaskCheckpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TaskReceived => "task_received",
            Self::SlotReserved => "slot_reserved",
            Self::ClaimRequested => "claim_requested",
            Self::Claimed => "claimed",
            Self::MetadataParsed => "metadata_parsed",
            Self::WorkspaceReady => "workspace_ready",
            Self::SessionReady => "session_ready",
            Self::GitHookInstalled => "git_hook_installed",
            Self::ProvidersLoaded => "providers_loaded",
            Self::ModelSelected => "model_selected",
            Self::AgentStarting => "agent_starting",
            Self::AgentRunning => "agent_running",
            Self::AgentDone => "agent_done",
            Self::SessionSaved => "session_saved",
            Self::CommitStaging => "commit_staging",
            Self::CommitCreated => "commit_created",
            Self::CommitPushing => "commit_pushing",
            Self::CommitPushed => "commit_pushed",
            Self::PrCreating => "pr_creating",
            Self::PrCreated => "pr_created",
            Self::Releasing => "releasing",
            Self::Released => "released",
            Self::Completed => "completed",
            Self::GracefulShutdown => "graceful_shutdown",
            Self::Failed => "failed",
        }
    }

    pub const ALL: &[TaskCheckpoint] = &[
        Self::TaskReceived,
        Self::SlotReserved,
        Self::ClaimRequested,
        Self::Claimed,
        Self::MetadataParsed,
        Self::WorkspaceReady,
        Self::SessionReady,
        Self::GitHookInstalled,
        Self::ProvidersLoaded,
        Self::ModelSelected,
        Self::AgentStarting,
        Self::AgentRunning,
        Self::AgentDone,
        Self::SessionSaved,
        Self::CommitStaging,
        Self::CommitCreated,
        Self::CommitPushing,
        Self::CommitPushed,
        Self::PrCreating,
        Self::PrCreated,
        Self::Releasing,
        Self::Released,
        Self::Completed,
        Self::GracefulShutdown,
        Self::Failed,
    ];
}

impl std::fmt::Display for TaskCheckpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct CheckpointEntry {
    pub checkpoint: TaskCheckpoint,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub elapsed_ms: u64,
    pub detail: Option<String>,
}

/// Shared progress state readable by the heartbeat task.
#[derive(Debug, Clone, serde::Serialize)]
pub struct TaskProgressState {
    pub task_id: Option<String>,
    pub current_checkpoint: Option<String>,
    pub elapsed_secs: f64,
    pub remaining_secs: f64,
    pub budget_pct_used: f64,
    pub checkpoints_reached: Vec<String>,
    pub last_detail: Option<String>,
}

impl Default for TaskProgressState {
    fn default() -> Self {
        Self {
            task_id: None,
            current_checkpoint: None,
            elapsed_secs: 0.0,
            remaining_secs: 0.0,
            budget_pct_used: 0.0,
            checkpoints_reached: Vec::new(),
            last_detail: None,
        }
    }
}

impl TaskProgressState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn clear(&mut self) {
        self.task_id = None;
        self.current_checkpoint = None;
        self.checkpoints_reached.clear();
        self.last_detail = None;
    }
}

/// Tracks the timeline of a task through the processing pipeline.
pub struct TaskTimeline {
    task_id: String,
    timeout_secs: u64,
    start: Instant,
    #[allow(dead_code)]
    start_utc: chrono::DateTime<chrono::Utc>,
    checkpoints: Vec<CheckpointEntry>,
    current: Option<TaskCheckpoint>,
    progress: Arc<Mutex<TaskProgressState>>,
}

impl TaskTimeline {
    pub fn new(task_id: &str, timeout_secs: u64) -> Self {
        let progress = Arc::new(Mutex::new(TaskProgressState {
            task_id: Some(task_id.to_string()),
            remaining_secs: timeout_secs as f64,
            ..Default::default()
        }));
        Self {
            task_id: task_id.to_string(),
            timeout_secs,
            start: Instant::now(),
            start_utc: chrono::Utc::now(),
            checkpoints: Vec::new(),
            current: None,
            progress,
        }
    }

    pub fn checkpoint(&mut self, cp: TaskCheckpoint) {
        self.checkpoint_with_detail(cp, None);
    }

    pub fn checkpoint_with_detail(&mut self, cp: TaskCheckpoint, detail: Option<String>) {
        let elapsed_ms = self.start.elapsed().as_millis() as u64;
        let now = chrono::Utc::now();
        let budget = self.budget_pct_used();

        tracing::info!(
            task_id = %self.task_id,
            checkpoint = cp.as_str(),
            elapsed_ms,
            timeout_secs = self.timeout_secs,
            budget_pct = format!("{:.1}%", budget),
            detail = detail.as_deref().unwrap_or(""),
            "[timeline] checkpoint reached"
        );

        if budget >= 90.0 {
            tracing::warn!(
                task_id = %self.task_id,
                checkpoint = cp.as_str(),
                budget_pct = format!("{:.1}%", budget),
                remaining_secs = format!("{:.1}", self.remaining_secs()),
                "DEADLINE APPROACHING - {:.0}% of time budget consumed", budget
            );
        } else if budget >= 75.0 {
            tracing::warn!(
                task_id = %self.task_id,
                budget_pct = format!("{:.1}%", budget),
                "Task consumed {:.0}% of time budget", budget
            );
        }

        self.current = Some(cp);
        self.checkpoints.push(CheckpointEntry {
            checkpoint: cp,
            timestamp: now,
            elapsed_ms,
            detail,
        });
    }

    pub fn progress_handle(&self) -> Arc<Mutex<TaskProgressState>> {
        Arc::clone(&self.progress)
    }

    pub async fn sync_progress(&self) {
        let mut state = self.progress.lock().await;
        state.current_checkpoint = self.current.map(|c| c.as_str().to_string());
        state.elapsed_secs = self.elapsed_secs();
        state.remaining_secs = self.remaining_secs();
        state.budget_pct_used = self.budget_pct_used();
        state.checkpoints_reached = self
            .checkpoints
            .iter()
            .map(|c| c.checkpoint.as_str().to_string())
            .collect();
        if let Some(last) = self.checkpoints.last() {
            state.last_detail = last.detail.clone();
        }
    }

    #[allow(dead_code)]
    pub async fn clear_progress(&self) {
        self.progress.lock().await.clear();
    }

    pub fn is_expired(&self) -> bool {
        self.elapsed_secs() >= self.timeout_secs as f64
    }
    pub fn is_near_deadline(&self) -> bool {
        self.budget_pct_used() >= 80.0
    }
    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
    pub fn remaining_secs(&self) -> f64 {
        (self.timeout_secs as f64 - self.elapsed_secs()).max(0.0)
    }
    pub fn budget_pct_used(&self) -> f64 {
        if self.timeout_secs == 0 {
            return 100.0;
        }
        (self.elapsed_secs() / self.timeout_secs as f64) * 100.0
    }
    #[allow(dead_code)]
    pub fn current(&self) -> Option<TaskCheckpoint> {
        self.current
    }
    pub fn reached(&self, cp: TaskCheckpoint) -> bool {
        self.checkpoints.iter().any(|e| e.checkpoint == cp)
    }

    /// Emit a structured diagnostics summary.
    pub fn emit_diagnostics(&self) {
        let total_ms = self.start.elapsed().as_millis();
        let reached: Vec<&str> = self
            .checkpoints
            .iter()
            .map(|e| e.checkpoint.as_str())
            .collect();
        let skipped: Vec<&str> = TaskCheckpoint::ALL
            .iter()
            .filter(|cp| !self.reached(**cp))
            .map(|cp| cp.as_str())
            .collect();
        let timeline: Vec<String> = self
            .checkpoints
            .iter()
            .map(|e| {
                let d = e
                    .detail
                    .as_deref()
                    .map(|d| format!("({})", d))
                    .unwrap_or_default();
                format!("{}@{}ms{}", e.checkpoint.as_str(), e.elapsed_ms, d)
            })
            .collect();
        let deltas: Vec<String> = (1..self.checkpoints.len())
            .map(|i| {
                let delta = self.checkpoints[i].elapsed_ms - self.checkpoints[i - 1].elapsed_ms;
                format!(
                    "{}->{}: {}ms",
                    self.checkpoints[i - 1].checkpoint.as_str(),
                    self.checkpoints[i].checkpoint.as_str(),
                    delta
                )
            })
            .collect();

        tracing::info!(
            task_id = %self.task_id,
            timeout_secs = self.timeout_secs,
            total_ms,
            budget_pct = format!("{:.1}%", self.budget_pct_used()),
            checkpoints_reached = ?reached,
            checkpoints_skipped = ?skipped,
            timeline = ?timeline,
            deltas = ?deltas,
            "[timeline] task diagnostics summary"
        );

        if self.is_expired() {
            tracing::warn!(
                task_id = %self.task_id,
                total_secs = format!("{:.1}", self.elapsed_secs()),
                timeout_secs = self.timeout_secs,
                last_checkpoint = self.current.map(|c| c.as_str()).unwrap_or("none"),
                "TASK EXCEEDED TIME BUDGET - last checkpoint: {}",
                self.current.map(|c| c.as_str()).unwrap_or("none")
            );
        }
    }

    /// Serialize diagnostics as JSON for inclusion in task output/release payloads.
    pub fn diagnostics_json(&self) -> serde_json::Value {
        let reached: Vec<serde_json::Value> = self
            .checkpoints
            .iter()
            .map(|e| {
                serde_json::json!({
                    "checkpoint": e.checkpoint.as_str(),
                    "elapsed_ms": e.elapsed_ms,
                    "timestamp": e.timestamp.to_rfc3339(),
                    "detail": e.detail,
                })
            })
            .collect();
        let skipped: Vec<&str> = TaskCheckpoint::ALL
            .iter()
            .filter(|cp| !self.reached(**cp))
            .map(|cp| cp.as_str())
            .collect();
        let deltas: Vec<serde_json::Value> = (1..self.checkpoints.len())
            .map(|i| {
                let delta = self.checkpoints[i].elapsed_ms - self.checkpoints[i - 1].elapsed_ms;
                serde_json::json!({
                    "from": self.checkpoints[i-1].checkpoint.as_str(),
                    "to": self.checkpoints[i].checkpoint.as_str(),
                    "delta_ms": delta,
                })
            })
            .collect();

        serde_json::json!({
            "task_id": self.task_id,
            "timeout_secs": self.timeout_secs,
            "total_elapsed_ms": self.start.elapsed().as_millis() as u64,
            "budget_pct_used": format!("{:.1}%", self.budget_pct_used()),
            "checkpoints_reached": reached,
            "checkpoints_skipped": skipped,
            "deltas": deltas,
            "expired": self.is_expired(),
        })
    }
}
