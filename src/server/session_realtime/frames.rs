//! Typed client and server WebSocket frames.

use crate::session::SessionResult;
use crate::session::thread_store::ThreadEvent;
use serde::{Deserialize, Serialize};

/// Commands accepted from an authenticated realtime client.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(super) enum ClientFrame {
    Prompt { message: String },
    Steer { request_id: String, message: String },
    Cancel,
}

/// Ordered status, event, and terminal frames sent to a client.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(super) enum ServerFrame {
    Ready { session_id: String },
    Event { event: ThreadEvent },
    Steering { request_id: String, accepted: bool },
    Result { result: SessionResult },
    Error { message: String },
}
