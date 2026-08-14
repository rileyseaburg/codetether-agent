//! Parsed `/mux` slash-command action type.

use std::path::PathBuf;

/// One resolved `/mux` operation ready for the control plane.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Action {
    Help,
    List,
    New {
        name: String,
        workspace: PathBuf,
        no_worktree: bool,
    },
    Window {
        name: String,
        workspace: PathBuf,
    },
    Select {
        name: String,
        id: u64,
    },
    Close {
        name: String,
        id: u64,
    },
    Kill {
        name: String,
    },
}
