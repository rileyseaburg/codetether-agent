//! Parent transcript inheritance for spawned-agent sessions.

use super::fork_turns::ForkTurns;
use crate::session::Session;
use anyhow::{Context, Result};

#[path = "fork_context/select.rs"]
mod select;
use select::run as select;

pub(in crate::tool::agent) async fn inherit(
    child: &mut Session,
    parent_id: Option<&str>,
    scope: ForkTurns,
) -> Result<()> {
    let Some(parent_id) = parent_id.filter(|_| scope != ForkTurns::None) else {
        return Ok(());
    };
    let parent = Session::load(parent_id)
        .await
        .context("failed to load parent transcript for fork")?;
    for message in select(parent.history(), scope) {
        child.add_message(message);
    }
    Ok(())
}

#[cfg(test)]
#[path = "fork_context_interruption_tests.rs"]
mod interruption_tests;
#[cfg(test)]
#[path = "fork_context_tests.rs"]
mod tests;
