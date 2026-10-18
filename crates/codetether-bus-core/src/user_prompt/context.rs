use std::path::Path;
/// Borrowed provenance for a human prompt bus message.
/// # Examples
/// ```
/// let context = codetether_bus_core::user_prompt::Context {
///     agent: "build", session_id: "session-1", workspace: None,
/// };
/// assert_eq!(context.agent, "build");
/// ```
pub struct Context<'a> {
    /// Agent identity used in topic and payload.
    pub agent: &'a str,
    /// Session identifier used in the payload.
    pub session_id: &'a str,
    /// Explicit workspace, or current-directory fallback when absent.
    pub workspace: Option<&'a Path>,
}
