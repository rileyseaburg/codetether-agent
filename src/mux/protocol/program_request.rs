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
    Tail {
        window_id: u64,
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

#[cfg(test)]
mod tests {
    #[test]
    fn tail_request_has_a_stable_wire_shape() {
        let value = serde_json::json!({"action": "tail", "window_id": 7});
        let request: super::ProgramRequest = serde_json::from_value(value).unwrap();
        assert!(matches!(
            request,
            super::ProgramRequest::Tail { window_id: 7 }
        ));
    }
}
