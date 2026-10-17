//! Application session binding for shared parent transcript inheritance.
use super::fork_turns::ForkTurns;
use crate::session::Session;
use anyhow::Result;
#[path = "fork_context/source.rs"]
mod source;
pub(in crate::tool::agent) async fn inherit(
    child: &mut Session,
    parent_id: Option<&str>,
    scope: ForkTurns,
) -> Result<()> {
    codetether_collaboration_history::inherit(child, parent_id, scope).await
}
