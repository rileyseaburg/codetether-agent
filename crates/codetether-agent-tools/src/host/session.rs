//! Native session information used during input enrichment.
use crate::*;
/// Session-scoped workspace, prior-context policy and message history.
/// # Implementors
/// Delegate to existing session enrichment without taking ownership of history.
pub trait SessionView: Send + Sync {
    /// The native session passed through preparation.
    type Session: Sync;
    /// Borrow the configured working directory when present.
    fn directory(session: &Self::Session) -> Option<&Path>;
    /// Apply workspace and prior-context fields to the parsed JSON input.
    fn enrich_input(input: &Value, cwd: &Path, session: &Self::Session) -> Value;
    /// Borrow conversation history for image-tool enrichment.
    fn messages(session: &Self::Session) -> &[Message];
}
