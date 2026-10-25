//! Wait for recording cancellation, its deadline, or a terminal stream failure.
use super::capture_errors::CaptureErrors;
use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

pub(crate) fn for_stop(
    duration: Duration,
    stop_flag: &AtomicBool,
    errors: &CaptureErrors,
) -> Result<()> {
    let started = Instant::now();
    loop {
        errors.check()?;
        let remaining = duration.saturating_sub(started.elapsed());
        if stop_flag.load(Ordering::Relaxed) || remaining.is_zero() {
            return Ok(());
        }
        std::thread::sleep(remaining.min(Duration::from_millis(25)));
    }
}
