//! Registry-shaped fixtures for in-process mux server tests.

use std::net::SocketAddr;

use chrono::Utc;

use crate::mux::model::{MuxRuntimeStatus, MuxSnapshot};
use crate::mux::registry::{MuxRecord, SessionTarget};

/// A `SessionTarget` describing `session` on an in-process server at `address`.
pub(super) fn target(state: MuxSnapshot, address: SocketAddr, session: &str) -> SessionTarget {
    let record = MuxRecord {
        key: crate::mux::registry::for_workspace(&state.workspace),
        address,
        token: "secret".into(),
        pid: 1,
        started_at: Utc::now(),
        state,
    };
    SessionTarget {
        record,
        session: session.into(),
    }
}

/// Minimal idle runtime report carrying only a durable session id.
pub(super) fn runtime(session_id: &str) -> MuxRuntimeStatus {
    MuxRuntimeStatus {
        session_id: session_id.into(),
        session_title: session_id.into(),
        processing: false,
        message_count: 0,
        current_tool: None,
        needs_interaction: false,
        lagging: false,
        principal: Default::default(),
    }
}
