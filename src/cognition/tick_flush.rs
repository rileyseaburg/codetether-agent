//! Tick publication of events, snapshots, and persisted state.

use super::loop_ctx::LoopCtx;
use super::tick_process::TickOutput;
use super::{buffers, persistence};

/// Publish this tick's events and snapshots to the bounded buffers.
pub(super) async fn flush(ctx: &LoopCtx, out: TickOutput) {
    for event in out.events {
        buffers::push_event_internal(&ctx.events, ctx.max_events, &ctx.event_tx, event).await;
    }
    for snapshot in out.snapshots {
        buffers::push_snapshot_internal(&ctx.snapshots, ctx.max_snapshots, snapshot).await;
    }
}

/// Insert newly minted proposals into the shared store.
pub(super) async fn store_proposals(ctx: &LoopCtx, out: &mut TickOutput) {
    if out.proposals.is_empty() {
        return;
    }
    let mut store = ctx.proposals.write().await;
    for proposal in out.proposals.drain(..) {
        store.insert(proposal.id.clone(), proposal);
    }
}

/// Persist cognition state after a Compress phase.
pub(super) async fn persist(ctx: &LoopCtx) {
    let _ = persistence::save_state(
        &ctx.personas,
        &ctx.proposals,
        &ctx.beliefs,
        &ctx.attention_queue,
        &ctx.workspace,
        &ctx.events,
        &ctx.snapshots,
    )
    .await;
}
