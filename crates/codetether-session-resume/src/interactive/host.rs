//! Storage and presentation boundaries for interactive session selection.
use crate::{continuation, loading::LoadedSession};
use std::{future::Future, path::Path};
/// Loaded session metadata and bus attachment operations.
pub trait Session: continuation::Session {
    /// Attach the process bus when no bus is attached yet.
    fn attach_global_bus_if_missing(&mut self);
    /// Saved automatic-edit preference.
    fn auto_apply_edits(&self) -> bool;
    /// Saved worktree preference.
    fn use_worktree(&self) -> bool;
    /// Display label used in the load status.
    fn display_label(&self) -> String;
    /// Number of messages retained in the loaded snapshot.
    fn visible_messages(&self) -> usize;
}
/// Native session storage and child-agent hydration.
pub trait Host {
    /// Session representation supplied by the application.
    type Session: Session;
    /// Load a session and its bounded-window metadata.
    /// # Errors
    /// Returns native loading or import failures.
    fn load(id: &str) -> impl Future<Output = anyhow::Result<LoadedSession<Self::Session>>>;
    /// Restore persisted child agents; failures remain best effort.
    fn hydrate(id: &str) -> impl Future<Output = ()>;
    /// Import workspace sessions and return imported/skipped counts.
    /// # Errors
    /// Returns archive discovery or import failures.
    fn import_workspace(cwd: &Path) -> impl Future<Output = anyhow::Result<(usize, usize)>>;
}
/// UI operations applied after loading or importing sessions.
pub trait View<S> {
    /// Restore automatic-edit and worktree preferences.
    fn set_preferences(&mut self, auto_apply: bool, worktree: bool);
    /// Select the loaded session identity.
    fn set_session_id(&mut self, id: String);
    /// Project the transcript and configure history pagination.
    fn sync_messages(&mut self, session: &S, source_id: &str, source_has_older: bool);
    /// Refresh the workspace session list.
    fn refresh(&mut self, cwd: &Path) -> impl Future<Output = ()>;
    /// Clear the picker filter.
    fn clear_filter(&mut self);
    /// Restore chat view and its saved scroll state.
    fn return_to_chat(&mut self);
    /// Replace the status line.
    fn set_status(&mut self, status: String);
}
