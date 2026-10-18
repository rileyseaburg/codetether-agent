//! Structured agent-turn operations accepted by a mux daemon.

use serde::{Deserialize, Serialize};

/// One task lifecycle request scoped to its authenticated mux session.
/// # Examples
/// ```
/// let request = codetether_mux_protocol::AgentRequest::Cancel { task_id: "t".into() };
/// assert!(matches!(request, codetether_mux_protocol::AgentRequest::Cancel { .. }));
/// ```
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum AgentRequest {
    Start {
        /// Agent task identifier.
        task_id: String,
        /// Prompt text for the agent.
        prompt: String,
        #[serde(default)]
        /// Optional durable session identifier.
        session_id: Option<String>,
        #[serde(default = "default_max_steps")]
        /// Maximum agent step count.
        max_steps: usize,
        #[serde(default)]
        /// Optional tool profile.
        tool_profile: Option<String>,
    },
    Read {
        /// Agent task identifier.
        task_id: String,
        /// Requested replay offset.
        offset: u64,
    },
    Cancel {
        /// Agent task identifier.
        task_id: String,
    },
}

fn default_max_steps() -> usize {
    6
}
