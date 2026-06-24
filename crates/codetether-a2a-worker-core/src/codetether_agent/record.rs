use super::state::SubAgentRunState;
use serde::{Deserialize, Serialize};

/// Durable diagnostic record for a spawned sub-agent.
///
/// The record is derived from archived CodeTether session metadata and can be
/// returned from `agent status` so restarts do not hide orphaned children.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_worker_core::codetether_agent::subagent_orphans::{SubAgentRunRecord, SubAgentRunState};
///
/// let record = SubAgentRunRecord::orphaned("auditor", "child-1", "created but idle");
/// assert_eq!(record.state, SubAgentRunState::Orphaned);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubAgentRunRecord {
    /// Human-readable sub-agent name from session provenance.
    pub agent_name: String,
    /// Child CodeTether session id.
    pub child_session_id: String,
    /// Durable state inferred for the child run.
    pub state: SubAgentRunState,
    /// Number of messages recorded in the child session.
    pub message_count: usize,
    /// Number of tool uses recorded in the child session.
    pub tool_use_count: usize,
    /// Optional heartbeat timestamp when a worker reported activity.
    pub last_heartbeat_at: Option<String>,
    /// Human-readable diagnostic reason.
    pub reason: String,
}

impl SubAgentRunRecord {
    /// Builds an orphaned sub-agent record with no worker activity.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_a2a_worker_core::codetether_agent::subagent_orphans::SubAgentRunRecord;
    ///
    /// let record = SubAgentRunRecord::orphaned("sql", "session", "no heartbeat");
    /// assert_eq!(record.reason, "no heartbeat");
    /// ```
    pub fn orphaned(
        agent_name: impl Into<String>,
        child_session_id: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            agent_name: agent_name.into(),
            child_session_id: child_session_id.into(),
            state: SubAgentRunState::Orphaned,
            message_count: 1,
            tool_use_count: 0,
            last_heartbeat_at: None,
            reason: reason.into(),
        }
    }
}
