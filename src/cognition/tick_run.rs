//! Bind shared tick sequencing to native thought processing and persistence.
mod host;
use super::loop_ctx::LoopCtx;
pub(super) async fn run_tick(ctx: &LoopCtx) {
    codetether_cognition_core::runtime::tick::run_tick(ctx, &host::Native).await;
}
