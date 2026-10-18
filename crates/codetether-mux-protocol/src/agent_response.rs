//! Structured agent-turn responses emitted by a mux daemon.

use serde::{Deserialize, Serialize};

/// One response from the mux-owned agent task registry.
/// # Examples
/// ```
/// let reply = codetether_mux_protocol::AgentResponse::Accepted { task_id: "t".into() };
/// assert!(matches!(reply, codetether_mux_protocol::AgentResponse::Accepted { .. }));
/// ```
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum AgentResponse {
    Accepted {
        /// Agent task identifier.
        task_id: String,
    },
    Output {
        /// Agent task identifier.
        task_id: String,
        /// Raw PTY or agent output bytes.
        data: Vec<u8>,
        /// Offset immediately after the returned bytes.
        next_offset: u64,
        /// Whether the task or program is still active.
        running: bool,
        /// Process exit status when complete.
        exit_code: Option<i32>,
    },
    Cancelled {
        /// Agent task identifier.
        task_id: String,
    },
    Error {
        /// Agent task identifier.
        task_id: Option<String>,
        /// Error diagnostic.
        message: String,
    },
}
