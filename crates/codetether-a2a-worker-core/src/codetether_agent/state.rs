use serde::{Deserialize, Serialize};

/// Durable lifecycle state for a CodeTether sub-agent run.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_worker_core::codetether_agent::subagent_orphans::SubAgentRunState;
/// assert_eq!(SubAgentRunState::Orphaned.as_str(), "Orphaned");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubAgentRunState {
    /// Parent recorded a spawn and created a child session.
    Created,
    /// Worker has been queued but has not sent a heartbeat.
    Queued,
    /// Child sent activity or a heartbeat.
    Running,
    /// Child produced a final result.
    Completed,
    /// Child failed with a concrete error.
    Failed,
    /// Child was cancelled by the parent or supervisor.
    Cancelled,
    /// Child exists but no worker activity was observed.
    Orphaned,
}

impl SubAgentRunState {
    /// Returns the stable display label for a sub-agent run state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_a2a_worker_core::codetether_agent::subagent_orphans::SubAgentRunState;
    /// assert_eq!(SubAgentRunState::Running.as_str(), "Running");
    /// ```
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "Created",
            Self::Queued => "Queued",
            Self::Running => "Running",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
            Self::Cancelled => "Cancelled",
            Self::Orphaned => "Orphaned",
        }
    }
}
