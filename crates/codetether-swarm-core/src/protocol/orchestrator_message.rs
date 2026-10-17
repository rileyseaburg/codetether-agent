use crate::SubTask;
use serde::{Deserialize, Serialize};

/// Message from orchestrator to sub-agent
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let message = codetether_swarm_core::OrchestratorMessage::Terminate { reason: "complete".into() };
/// assert!(matches!(message, codetether_swarm_core::OrchestratorMessage::Terminate { .. }));
/// ```
pub enum OrchestratorMessage {
    /// Start execution
    Start { subtask: Box<SubTask> },

    /// Provide resource
    Resource {
        resource_type: String,
        resource_id: String,
        content: String,
    },

    /// Terminate execution
    Terminate { reason: String },

    /// Context update (from completed dependency)
    ContextUpdate {
        dependency_id: String,
        result: String,
    },
}
