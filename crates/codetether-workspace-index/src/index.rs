//! JSON representation of the workspace-to-session sidecar.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Canonical workspace paths mapped to their most-recent session IDs.
/// # Examples
/// ```
/// use codetether_workspace_index::WorkspaceIndex;
/// let index = WorkspaceIndex::default();
/// assert!(index.entries.is_empty());
/// ```
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WorkspaceIndex {
    /// Canonical workspace path as a string mapped to its session UUID.
    pub entries: HashMap<String, String>,
}
