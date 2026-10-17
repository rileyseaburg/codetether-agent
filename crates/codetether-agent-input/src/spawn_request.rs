//! Validated borrowed spawn fields and owned workspace snapshot.
use codetether_collaboration_history::ForkTurns;
use std::path::PathBuf;
/// Required spawn fields borrowed from a parsed payload.
/// # Examples
/// ```rust
/// use codetether_agent_input::{Params, SpawnRequest};
/// let params: Params = serde_json::from_str(r#"{"action":"spawn","name":"child","instructions":"review","model":"provider/model"}"#).unwrap();
/// let request = SpawnRequest::from_params(&params).unwrap();
/// assert_eq!(request.name, "child");
/// ```
pub struct SpawnRequest<'a> {
    /// Required child display name.
    pub name: &'a str,
    /// Required task instructions.
    pub instructions: &'a str,
    /// Explicit model or current-parent fallback.
    pub model: &'a str,
    /// Whether this child is ephemeral.
    pub ephemeral: bool,
    /// Effective asynchronous dispatch flag.
    pub detach: bool,
    /// Owned parent workspace snapshot.
    pub parent_workspace: Option<PathBuf>,
    /// Borrowed owner session identity.
    pub parent_session_id: Option<&'a str>,
    /// Validated parent-history inheritance scope.
    pub fork_turns: ForkTurns,
}
