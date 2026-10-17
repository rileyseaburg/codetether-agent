//! SubTask execution contract.
use super::*;

/// A sub-task that can be executed by a sub-agent
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let task = codetether_swarm_core::SubTask::new("inspect", "Inspect files");
/// assert_eq!(task.status, codetether_swarm_core::SubTaskStatus::Pending);
/// ```
pub struct SubTask {
    /// Unique identifier
    pub id: String,

    /// Human-readable name/description
    pub name: String,

    /// The task instruction for the sub-agent
    pub instruction: String,

    /// Specialty/domain for this subtask
    pub specialty: Option<String>,

    /// Dependencies on other subtasks (by ID)
    pub dependencies: Vec<String>,

    /// Priority (higher = more important)
    pub priority: i32,

    /// Maximum steps allowed for this subtask
    pub max_steps: usize,

    /// Input context from parent or dependencies
    pub context: SubTaskContext,

    /// Current status
    pub status: SubTaskStatus,

    /// Assigned sub-agent ID
    pub assigned_agent: Option<String>,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Completion timestamp
    pub completed_at: Option<DateTime<Utc>>,

    /// Stage in the execution plan (0 = can run immediately)
    pub stage: usize,

    /// Model-suggested worktree preference. This is advisory, not
    /// authoritative: mutating instructions still require isolation even
    /// when model metadata claims the task is read-only.
    #[serde(default)]
    pub needs_worktree: Option<bool>,
}
