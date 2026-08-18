//! Proposal minting from a Reflect-phase thought.

use super::loop_ctx::LoopCtx;
use super::tick_process::TickOutput;
use super::{ThoughtResult, ThoughtWorkItem, proposal_create, proposal_event};

/// A proposal is minted every 8th thought, offset into the Reflect phase.
const EVERY: u64 = 8;
const OFFSET: u64 = 2;

/// Whether this thought count is a proposal checkpoint.
pub(super) fn is_checkpoint(thought_count: u64) -> bool {
    thought_count % EVERY == OFFSET
}

/// Mint a proposal and queue its creation event.
pub(super) async fn propose(
    ctx: &LoopCtx,
    work: &ThoughtWorkItem,
    thought: &ThoughtResult,
    active_persona_count: usize,
    out: &mut TickOutput,
) {
    let governance = ctx.governance.read().await;
    let proposal =
        proposal_create::build_proposal(work, thought, &governance, active_persona_count);
    out.events
        .push(proposal_event::created_event(work, thought, &proposal));
    out.proposals.push(proposal);
}
