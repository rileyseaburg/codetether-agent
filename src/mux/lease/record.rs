//! Serializable lease records returned to authenticated mux clients.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Exclusive ownership of one path inside a mux-managed workspace.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(crate) struct WorktreeLease {
    pub owner: String,
    pub agent: String,
    pub workspace: PathBuf,
    pub path: PathBuf,
    pub expires_at_ms: i64,
}
