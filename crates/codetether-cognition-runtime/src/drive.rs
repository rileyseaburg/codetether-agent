//! Cadence driving with application thought-processing hooks.
use codetether_cognition_core::runtime::{LoopContext, tick::Host};

/// Drive cognition ticks until the context's running flag is cleared.
/// # Arguments
/// * `ctx` — Owned handles for the background cognition loop.
/// * `host` — Application thought processing and tick persistence.
/// # Returns
/// Completes when the shared cadence driver exits.
/// # Examples
/// ```no_run
/// # async fn run<T, R, H: codetether_cognition_core::runtime::tick::Host<T, R>>(ctx: codetether_cognition_core::runtime::LoopContext<T, R>, host: H) {
/// codetether_cognition_runtime::drive(ctx, host).await;
/// # }
/// ```
pub async fn drive<T, R, H: Host<T, R>>(ctx: LoopContext<T, R>, host: H) {
    codetether_cognition_core::driver::drive(&ctx.running, &ctx.loop_interval_ms, || {
        run_tick(&ctx, &host)
    })
    .await;
}

/// Run one shared cognition tick using application processing hooks.
/// # Arguments
/// * `ctx` — Borrowed live cognition stores and configuration.
/// * `host` — Application thought processing and tick persistence.
/// # Returns
/// Completes after the shared tick's publication and optional persistence.
/// # Examples
/// ```no_run
/// # async fn tick<T, R, H: codetether_cognition_core::runtime::tick::Host<T, R>>(ctx: &codetether_cognition_core::runtime::LoopContext<T, R>, host: &H) {
/// codetether_cognition_runtime::run_tick(ctx, host).await;
/// # }
/// ```
pub async fn run_tick<T, R, H: Host<T, R>>(ctx: &LoopContext<T, R>, host: &H) {
    codetether_cognition_core::runtime::tick::run_tick(ctx, host).await;
}
