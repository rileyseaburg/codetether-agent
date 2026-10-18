//! Canonical workspace lookup and mapping updates.
use crate::WorkspaceIndex;
use std::path::Path;
impl WorkspaceIndex {
    /// Look up `workspace` (canonical path) and return the session id.
    /// # Arguments
    /// * `workspace` — Canonical workspace path used as the index key.
    /// # Returns
    /// The indexed session ID, if present.
    /// # Examples
    /// ```
    /// use codetether_workspace_index::WorkspaceIndex;
    /// let index = WorkspaceIndex::default();
    /// assert_eq!(index.get(std::path::Path::new("/project")), None);
    /// ```
    pub fn get(&self, workspace: &Path) -> Option<&str> {
        self.entries
            .get(workspace.to_string_lossy().as_ref())
            .map(String::as_str)
    }

    /// Record a session id for a canonical workspace path.
    /// # Arguments
    /// * `workspace` — Canonical workspace path used as the index key.
    /// * `session_id` — ID to record as the most-recent session.
    /// # Returns
    /// Updates the in-memory mapping without writing it to disk.
    /// # Examples
    /// ```
    /// use codetether_workspace_index::WorkspaceIndex;
    /// let mut index = WorkspaceIndex::default();
    /// let path = std::path::Path::new("/project");
    /// index.set(path, "session-id");
    /// assert_eq!(index.get(path), Some("session-id"));
    /// ```
    pub fn set(&mut self, workspace: &Path, session_id: &str) {
        self.entries.insert(
            workspace.to_string_lossy().into_owned(),
            session_id.to_string(),
        );
    }
}
