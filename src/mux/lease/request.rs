//! Authenticated requests accepted by the mux lease authority.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// One shared-worktree coordination operation.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub(in crate::mux) enum CoordinationRequest {
    Acquire {
        owner: String,
        agent: String,
        workspace: PathBuf,
        paths: Vec<PathBuf>,
    },
    Renew {
        owner: String,
    },
    Release {
        owner: String,
    },
    Snapshot,
}
