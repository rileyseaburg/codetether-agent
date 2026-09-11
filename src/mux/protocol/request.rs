//! Client-to-server mux operations.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::mux::model::MuxRuntimeStatus;

use super::{AgentRequest, ProgramRequest};

/// One authenticated mux control request.
///
/// A connection binds to one session at [`ClientRequest::Authenticate`];
/// window, program, agent, and runtime requests then act only on that
/// session. [`ClientRequest::Coordinate`] is server-wide because leases guard
/// the checkout every session shares.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(in crate::mux) enum ClientRequest {
    Authenticate {
        token: String,
        /// Session this connection operates on; `None` selects server-only
        /// operations (`CreateSession`, `Coordinate`, `Shutdown`, `Snapshot`).
        #[serde(default)]
        session: Option<String>,
    },
    Snapshot,
    CreateSession {
        name: String,
        workspace: PathBuf,
    },
    CloseSession {
        name: String,
    },
    CreateWindow {
        workspace: PathBuf,
    },
    SelectWindow {
        id: u64,
    },
    CloseWindow {
        id: u64,
    },
    ChangeDirectory {
        workspace: PathBuf,
    },
    Program {
        request: ProgramRequest,
    },
    Agent {
        request: AgentRequest,
    },
    ReportRuntime {
        status: Option<MuxRuntimeStatus>,
    },
    Coordinate {
        request: crate::mux::lease::CoordinationRequest,
    },
    Detach,
    Shutdown,
}
