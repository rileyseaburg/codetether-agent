//! PTY operations nested under the mux client request envelope.

use serde::{Deserialize, Serialize};

/// One operation targeting a server-owned PTY program.
/// # Examples
/// ```
/// let request = codetether_mux_protocol::ProgramRequest::Tail { window_id: 1 };
/// assert!(matches!(request, codetether_mux_protocol::ProgramRequest::Tail { .. }));
/// ```
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ProgramRequest {
    Start {
        /// Target window ID.
        window_id: u64,
        /// Program command line.
        command: String,
        /// Terminal width in columns.
        columns: u16,
        /// Terminal height in rows.
        rows: u16,
    },
    Attach {
        /// Target window ID.
        window_id: u64,
        /// Terminal width in columns.
        columns: u16,
        /// Terminal height in rows.
        rows: u16,
    },
    Tail {
        /// Target window ID.
        window_id: u64,
    },
    Input {
        /// Target window ID.
        window_id: u64,
        /// Raw PTY or agent output bytes.
        data: Vec<u8>,
    },
    Steer {
        /// Semantic steering text.
        text: String,
    },
    Read {
        /// Target window ID.
        window_id: u64,
        /// Requested replay offset.
        offset: u64,
    },
    Resize {
        /// Target window ID.
        window_id: u64,
        /// Terminal width in columns.
        columns: u16,
        /// Terminal height in rows.
        rows: u16,
    },
}
#[cfg(test)]
#[path = "program_request_tests.rs"]
mod tests;
