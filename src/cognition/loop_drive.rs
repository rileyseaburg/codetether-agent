//! The perpetual cognition loop driver.

use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::time::Instant;

use super::loop_ctx::LoopCtx;
use super::tick_run::run_tick;

/// Minimum loop interval, in milliseconds.
const MIN_INTERVAL_MS: u64 = 100;

/// Drive ticks on a fixed cadence until `running` clears.
///
/// A tick that overruns its interval resets the schedule rather than
/// accumulating debt and busy-looping.
pub(super) async fn drive(ctx: LoopCtx) {
    let mut next_tick = Instant::now();
    while ctx.running.load(Ordering::SeqCst) {
        if Instant::now() < next_tick {
            tokio::time::sleep_until(next_tick).await;
        }
        if !ctx.running.load(Ordering::SeqCst) {
            break;
        }

        run_tick(&ctx).await;

        let interval =
            Duration::from_millis((*ctx.loop_interval_ms.read().await).max(MIN_INTERVAL_MS));
        next_tick += interval;
        let completed = Instant::now();
        if completed > next_tick {
            next_tick = completed;
        }
    }
}
