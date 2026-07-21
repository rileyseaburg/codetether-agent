//! Client-to-server mux operations.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::mux::model::MuxRuntimeStatus;

use super::{AgentRequest, ProgramRequest};

/// One authenticated mux control request.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(in crate::mux) enum ClientRequest {
    Authenticate {
        token: String,
    },
    Snapshot,
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
