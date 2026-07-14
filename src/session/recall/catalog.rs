//! Workspace-to-session recall catalog.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const SCHEMA_VERSION: u8 = 1;

/// Small workspace manifest pointing at per-session recall sidecars.
#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct RecallCatalog {
    pub schema_version: u8,
    pub workspace: PathBuf,
    pub session_ids: Vec<String>,
}

impl RecallCatalog {
    pub(crate) fn new(workspace: PathBuf) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            workspace,
            session_ids: Vec::new(),
        }
    }

    pub(crate) fn accepts(&self, workspace: &std::path::Path) -> bool {
        self.schema_version == SCHEMA_VERSION && self.workspace == workspace
    }

    pub(crate) fn insert(&mut self, session_id: &str) {
        if !self.session_ids.iter().any(|id| id == session_id) {
            self.session_ids.push(session_id.to_string());
        }
    }

    pub(crate) fn remove(&mut self, session_id: &str) {
        self.session_ids.retain(|id| id != session_id);
    }
}
