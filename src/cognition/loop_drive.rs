//! Supply the application tick to the shared cadence driver.
use super::{loop_ctx::LoopCtx, tick_run::run_tick};

pub(super) async fn drive(ctx: LoopCtx) {
    codetether_cognition_core::driver::drive(&ctx.running, &ctx.loop_interval_ms, || {
        run_tick(&ctx)
    })
    .await;
}
