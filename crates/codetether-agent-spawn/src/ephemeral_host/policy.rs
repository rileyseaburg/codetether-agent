//! Native task classification and repository-aware prompt rendering.
use codetether_agent_input::SpawnRequest;
use std::path::Path;
/// Task intent and prompt policy used during one-shot setup.
/// # Implementors
/// Applications preserve their current task classifier and session instructions.
pub trait EphemeralPolicy {
    /// Return read-only status and whether file changes are expected, in that order.
    fn intent(request: &SpawnRequest<'_>) -> (bool, bool);
    /// Render the existing direct-worker prompt before isolation guidance is appended.
    fn prompt(
        request: &SpawnRequest<'_>,
        workspace: &Path,
        model: &str,
        read_only: bool,
        expects_changes: bool,
    ) -> String;
}
