use crate::{
    runtime::LoopContext,
    thought::{TickOutput, buffers},
};
/// Publish this tick's events and snapshots to the bounded buffers.
pub(super) async fn flush<T, R>(ctx: &LoopContext<T, R>, out: TickOutput) {
    for event in out.events {
        buffers::push_event_internal(&ctx.events, ctx.max_events, &ctx.event_tx, event).await;
    }
    for snapshot in out.snapshots {
        buffers::push_snapshot_internal(&ctx.snapshots, ctx.max_snapshots, snapshot).await;
    }
}

/// Insert newly minted proposals into the shared store.
pub(super) async fn store_proposals<T, R>(ctx: &LoopContext<T, R>, out: &mut TickOutput) {
    if out.proposals.is_empty() {
        return;
    }
    let mut store = ctx.proposals.write().await;
    for proposal in out.proposals.drain(..) {
        store.insert(proposal.id.clone(), proposal);
    }
}
