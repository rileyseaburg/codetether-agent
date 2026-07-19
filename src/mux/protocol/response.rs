//! Server-to-client mux responses.

use serde::{Deserialize, Serialize};

use crate::mux::model::MuxSnapshot;

/// One mux control response.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(in crate::mux) enum ServerResponse {
    Authenticated {
        version: u16,
    },
    Snapshot {
        state: MuxSnapshot,
    },
    ProgramAttached {
        window_id: u64,
        offset: u64,
        #[serde(default)]
        replay_until: u64,
        #[serde(default)]
        alternate_screen: bool,
    },
    ProgramOutput {
        data: Vec<u8>,
        next_offset: u64,
        running: bool,
    },
    Coordination {
        reply: crate::mux::lease::CoordinationReply,
    },
    Acknowledged,
    Detached,
    ShuttingDown,
    Error {
        message: String,
    },
}
