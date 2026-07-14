//! Client-to-server mux operations.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

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
    StartProgram {
        window_id: u64,
        command: String,
        columns: u16,
        rows: u16,
    },
    AttachProgram {
        window_id: u64,
        columns: u16,
        rows: u16,
    },
    ProgramInput {
        window_id: u64,
        data: Vec<u8>,
    },
    ReadProgram {
        window_id: u64,
        offset: u64,
    },
    ResizeProgram {
        window_id: u64,
        columns: u16,
        rows: u16,
    },
    Detach,
    Shutdown,
}
