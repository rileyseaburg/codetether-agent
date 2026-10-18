//! Workspace-to-session recall catalog.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const SCHEMA_VERSION: u8 = 1;

/// Small workspace manifest pointing at per-session recall sidecars.
/// # Examples
/// ```
/// let mut catalog = codetether_recall_store::RecallCatalog::new("/tmp".into());
/// catalog.insert("session-id");
/// assert_eq!(catalog.session_ids, ["session-id"]);
/// ```
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RecallCatalog {
    /// Persisted catalog format version.
    pub schema_version: u8,
    /// Canonical workspace represented by this catalog.
    pub workspace: PathBuf,
    /// Unique session IDs in insertion order.
    pub session_ids: Vec<String>,
}

impl RecallCatalog {
    /// Construct an empty catalog using the supported schema.
    pub fn new(workspace: PathBuf) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            workspace,
            session_ids: Vec::new(),
        }
    }

    /// Check schema and canonical workspace identity.
    pub fn accepts(&self, workspace: &std::path::Path) -> bool {
        self.schema_version == SCHEMA_VERSION && self.workspace == workspace
    }

    /// Add a session ID if it is not already present.
    pub fn insert(&mut self, session_id: &str) {
        if !self.session_ids.iter().any(|id| id == session_id) {
            self.session_ids.push(session_id.to_string());
        }
    }

    /// Remove all entries with the supplied session ID.
    pub fn remove(&mut self, session_id: &str) {
        self.session_ids.retain(|id| id != session_id);
    }
}
