//! Structured agent-turn operations accepted by a mux daemon.

use serde::{Deserialize, Serialize};

/// One task lifecycle request scoped to its authenticated mux session.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub(in crate::mux) enum AgentRequest {
    Start {
        task_id: String,
        prompt: String,
        #[serde(default)]
        session_id: Option<String>,
        #[serde(default = "default_max_steps")]
        max_steps: usize,
        #[serde(default)]
        tool_profile: Option<String>,
    },
    Read {
        task_id: String,
        offset: u64,
    },
    Cancel {
        task_id: String,
    },
}

fn default_max_steps() -> usize {
    6
}
