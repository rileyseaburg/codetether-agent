//! Application-owned mapping from a session identity to its task journal.
use anyhow::Result;
use codetether_goal_core::TaskLog;
/// Resolves journals without coupling goal execution to native session storage.
/// # Implementors
/// Native applications preserve their existing session-ID validation and paths.
pub trait Journal {
    /// Resolve a session journal, returning native path/identity errors unchanged.
    fn for_session(session_id: &str) -> Result<TaskLog>;
}
