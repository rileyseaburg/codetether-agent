//! Parent transcript inheritance for spawned-agent sessions.

use crate::{ForkTurns, HistorySource};
use anyhow::{Context, Result};

use crate::select::run as select;

/// Append the selected parent transcript to a child.
/// # Arguments
/// * `child` — Destination session using its normal append behavior.
/// * `parent_id` — Optional parent identity; absence skips inheritance.
/// * `scope` — Requested user-turn scope; none skips loading.
/// # Returns
/// Unit after appending the selected messages.
/// # Errors
/// Propagates parent-load errors with the original fork context.
/// # Examples
/// Use [HistorySource] to bind this operation to an application session.
pub async fn inherit<S: HistorySource>(
    child: &mut S,
    parent_id: Option<&str>,
    scope: ForkTurns,
) -> Result<()> {
    let Some(parent_id) = parent_id.filter(|_| scope != ForkTurns::None) else {
        return Ok(());
    };
    let parent = S::load(parent_id)
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
