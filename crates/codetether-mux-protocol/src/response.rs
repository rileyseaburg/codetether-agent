//! Server-to-client mux responses.

use serde::{Deserialize, Serialize};

use super::AgentResponse;
use codetether_mux_model::MuxSnapshot;

/// One mux control response.
/// # Examples
/// ```
/// let response = codetether_mux_protocol::ServerResponse::Acknowledged;
/// assert!(matches!(response, codetether_mux_protocol::ServerResponse::Acknowledged));
/// ```
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerResponse {
    Authenticated {
        /// Negotiated protocol version.
        version: u16,
    },
    Snapshot {
        /// Current server snapshot.
        state: MuxSnapshot,
    },
    ProgramAttached {
        /// Target window ID.
        window_id: u64,
        /// Requested replay offset.
        offset: u64,
        #[serde(default)]
        /// End of the initial replay range.
        replay_until: u64,
        #[serde(default)]
        /// Whether the program uses its alternate screen.
        alternate_screen: bool,
    },
    ProgramOutput {
        /// Raw PTY or agent output bytes.
        data: Vec<u8>,
        /// Offset immediately after the returned bytes.
        next_offset: u64,
        /// Whether the task or program is still active.
        running: bool,
    },
    Coordination {
        /// Authoritative coordination outcome.
        reply: codetether_worktree_lease::CoordinationReply,
    },
    Agent {
        /// Nested agent response.
        response: AgentResponse,
    },
    Acknowledged,
    Detached,
    ShuttingDown,
    Error {
        /// Error diagnostic.
        message: String,
    },
}
