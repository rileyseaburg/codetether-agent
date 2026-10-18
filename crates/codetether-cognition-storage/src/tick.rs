//! Best-effort persistence of the stores owned by a cognition loop.
use codetether_cognition_core::runtime::LoopContext;
/// Persist cognition state after a Compress phase, retaining best-effort errors.
/// # Arguments
/// * `ctx` — Stores captured in their established persistence order.
/// # Returns
/// Completes after attempting the existing shared save operation.
/// # Examples
/// ```no_run
/// # async fn save<T, R>(ctx: &codetether_cognition_core::runtime::LoopContext<T, R>) {
/// codetether_cognition_storage::persist_tick(ctx).await;
/// # }
/// ```
pub async fn persist_tick<T, R>(ctx: &LoopContext<T, R>) {
    let _ = super::save_state(
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
