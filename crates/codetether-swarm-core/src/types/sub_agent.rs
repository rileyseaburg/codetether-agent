//! SubAgent execution contract.
use super::*;

/// A sub-agent executing a subtask
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let agent = codetether_swarm_core::SubAgent::new("worker", "coding", "task-1", "model", "provider");
/// assert_eq!(agent.steps, 0);
/// ```
pub struct SubAgent {
    /// Unique identifier
    pub id: String,

    /// Display name
    pub name: String,

    /// Specialty/role (e.g., "AI Researcher", "Code Writer", "Fact Checker")
    pub specialty: String,

    /// The subtask this agent is working on
    pub subtask_id: String,

    /// Current status
    pub status: SubAgentStatus,

    /// Number of steps taken
    pub steps: usize,

    /// Tool calls made
    pub tool_calls: Vec<ToolCallRecord>,

    /// Accumulated output
    pub output: String,

    /// Model being used
    pub model: String,

    /// Provider
    pub provider: String,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last activity timestamp
    pub last_active: DateTime<Utc>,
}
