//! Native persistence binding for shared cognition ticks.
use super::{loop_ctx::LoopCtx, persistence};
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
