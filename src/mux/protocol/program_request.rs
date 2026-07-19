//! PTY operations nested under the mux client request envelope.

use serde::{Deserialize, Serialize};

/// One operation targeting a server-owned PTY program.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub(in crate::mux) enum ProgramRequest {
    Start {
        window_id: u64,
        command: String,
        columns: u16,
        rows: u16,
    },
    Attach {
        window_id: u64,
        columns: u16,
        rows: u16,
    },
    Input {
        window_id: u64,
        data: Vec<u8>,
    },
    Read {
        window_id: u64,
        offset: u64,
    },
    Resize {
        window_id: u64,
        columns: u16,
        rows: u16,
    },
}
