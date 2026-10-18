use crate::proposal::build as proposal_create;
use crate::proposal::build as proposal_event;
use crate::{ThoughtResult, ThoughtWorkItem, runtime::LoopContext, thought::TickOutput};
/// A proposal is minted every 8th thought, offset into the Reflect phase.
const EVERY: u64 = 8;
const OFFSET: u64 = 2;

/// Whether this thought count is a proposal checkpoint.
pub(super) fn is_checkpoint(thought_count: u64) -> bool {
    thought_count % EVERY == OFFSET
}

/// Mint a proposal and queue its creation event.
pub(super) async fn propose<T, R>(
    ctx: &LoopContext<T, R>,
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
