//! Typed client and server WebSocket frames.

use codetether_session_events::SessionResult;
use codetether_thread_store::ThreadEvent;
use serde::{Deserialize, Serialize};

/// Commands accepted from an authenticated realtime client.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
/// # Examples
/// ```
/// let _frame = codetether_session_realtime::frames::ClientFrame::Cancel;
/// ```
pub enum ClientFrame {
    Prompt {
        message: String,
        model: Option<String>,
    },
    Steer {
        request_id: String,
        message: String,
    },
    Cancel,
}

/// Ordered status, event, and terminal frames sent to a client.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
/// # Examples
/// ```
/// let _frame = codetether_session_realtime::frames::ServerFrame::Ready { session_id: "session-1".into(), model_selection: true };
/// ```
pub enum ServerFrame {
    Ready {
        session_id: String,
        model_selection: bool,
    },
    Event {
        event: ThreadEvent,
    },
    Steering {
        request_id: String,
        accepted: bool,
    },
    Result {
        result: SessionResult,
    },
    Error {
        message: String,
    },
}
