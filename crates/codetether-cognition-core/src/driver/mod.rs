//! Fixed-cadence cognition driving with a host-supplied tick operation.
//!
//! [drive] owns stop checks and overrun rescheduling. The caller owns tick
//! composition, runtime state and the task that polls this future.
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::{sync::RwLock, time::Instant};

/// Minimum loop interval, in milliseconds.
const MIN_INTERVAL_MS: u64 = 100;

/// Run ticks until the shared running flag clears.
///
/// # Arguments
/// * `running` - Stop flag checked before and after scheduled sleeping.
/// * `loop_interval_ms` - Interval reread after each tick, with the existing minimum.
/// * `tick` - Tick operation supplied by the host.
///
/// # Returns
/// Unit when the running flag clears. Overrunning ticks reset the next deadline.
///
/// # Examples
/// ```no_run
/// # async fn example() {
/// use std::sync::atomic::AtomicBool;
/// use tokio::sync::RwLock;
/// let running = AtomicBool::new(false);
/// codetether_cognition_core::driver::drive(&running, &RwLock::new(1000), || async {}).await;
/// # }
/// ```
pub async fn drive<F, T>(running: &AtomicBool, loop_interval_ms: &RwLock<u64>, mut tick: F)
where
    F: FnMut() -> T,
    T: Future<Output = ()>,
{
    let mut next_tick = Instant::now();
    while running.load(Ordering::SeqCst) {
        if Instant::now() < next_tick {
            tokio::time::sleep_until(next_tick).await;
        }
        if !running.load(Ordering::SeqCst) {
            break;
        }

        tick().await;

        let interval = Duration::from_millis((*loop_interval_ms.read().await).max(MIN_INTERVAL_MS));
        next_tick += interval;
        let completed = Instant::now();
        if completed > next_tick {
            next_tick = completed;
        }
    }
}
