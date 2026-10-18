//! On-disk persistence: save, load, delete, and directory lookup.

use anyhow::Result;

use super::tail_load::TailLoad;
use super::types::Session;

mod lifecycle_host;
#[cfg(test)]
mod location;
mod paths;
#[cfg(test)]
#[path = "save_guard.rs"]
mod save_guard;
#[cfg(test)]
#[path = "persistence_snapshot.rs"]
mod snapshot;
#[path = "workspace_resolve.rs"]
mod workspace_resolve;

impl Session {
    /// Load an existing session by its UUID.
    ///
    /// # Errors
    ///
    /// Returns an error if the session file does not exist, exceeds the
    /// sanity size cap, or the JSON is malformed.
    pub async fn load(id: &str) -> Result<Self> {
        codetether_session_store::lifecycle::load::<Self>(id).await
    }

    /// Load the most recent session, optionally scoped to a workspace
    /// directory.
    ///
    /// When `workspace` is [`Some`], only considers sessions created in that
    /// directory. When [`None`], returns the most recent session globally.
    ///
    /// # Errors
    ///
    /// Returns an error if no sessions exist (or, with `workspace` set,
    /// none match the requested directory).
    pub async fn last_for_directory(workspace: Option<&std::path::Path>) -> Result<Self> {
        Self::last_for_directory_tail(workspace, usize::MAX)
            .await
            .map(|t| t.session)
    }

    /// Like [`Self::last_for_directory`] but keeps only the last `window`
    /// messages and tool uses in memory, returning a [`TailLoad`] with the
    /// number of entries that were dropped. Use this when resuming very
    /// large sessions where the full transcript would exhaust memory.
    ///
    /// Listing combines database headers with unmigrated legacy entries.
    /// The selected session is resumed through the indexed store rather than
    /// parsing the transcript to discard its prefix.
    pub async fn last_for_directory_tail(
        workspace: Option<&std::path::Path>,
        window: usize,
    ) -> Result<TailLoad> {
        codetether_session_store::lifecycle::last_for_directory_tail::<Self>(workspace, window)
            .await
    }

    /// Load the most recent session globally (unscoped).
    ///
    /// Kept for legacy compatibility; prefer
    /// [`Session::last_for_directory`].
    pub async fn last() -> Result<Self> {
        Self::last_for_directory(None).await
    }

    /// Atomically flush changed records and metadata to the indexed session store.
    ///
    /// Unchanged sessions do not write to disk or enqueue background work. Changed
    /// sessions encode only dirty suffixes; the transcript prefix is never cloned,
    /// hashed, or rewritten. SQLite transactions run on a blocking worker.
    ///
    /// # Errors
    /// Returns storage errors or `SESSION_REVISION_CONFLICT` for a stale writer.
    /// Reload a conflicting session instead of silently replacing newer records.
    /// A legacy JSON file must be imported through the load API before saving.
    ///
    /// For a full JSON snapshot use [`Self::export_json`] explicitly. Per-session
    /// `.json` files are now small locators, not the canonical transcript.
    ///
    pub async fn save(&self) -> Result<()> {
        codetether_session_store::lifecycle::save::<Self>(self).await
    }

    /// Delete a session file by ID. No-op if the file does not exist.
    pub async fn delete(id: &str) -> Result<()> {
        codetether_session_store::lifecycle::delete::<Self>(id).await
    }
}
