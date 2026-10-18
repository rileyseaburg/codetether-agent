//! Event and transcript publication for completed tool calls.

use super::{super::Runner, call::Call, outcome::Outcome};

/// Publishes and records a completed executable tool result.
pub(super) async fn complete(
    runner: &mut Runner<'_>,
    step: usize,
    call: &Call,
    outcome: Outcome,
) -> anyhow::Result<()> {
    codetether_prompt_loop::tool_publication::complete(runner, step, call, outcome).await
}

#[path = "publication/host.rs"]
mod host;
