//! Structured agent-turn responses emitted by a mux daemon.

use serde::{Deserialize, Serialize};

/// One response from the mux-owned agent task registry.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub(in crate::mux) enum AgentResponse {
    Accepted {
        task_id: String,
    },
    Output {
        task_id: String,
        data: Vec<u8>,
        next_offset: u64,
        running: bool,
        exit_code: Option<i32>,
    },
    Cancelled {
        task_id: String,
    },
    Error {
        task_id: Option<String>,
        message: String,
    },
}
