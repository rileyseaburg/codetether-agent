//! Std concurrency gate for background RLM jobs.

use std::sync::atomic::{AtomicUsize, Ordering};

static RUNNING: AtomicUsize = AtomicUsize::new(0);

pub(super) struct Permit;

pub(super) fn try_acquire() -> Option<Permit> {
    let max = max_jobs();
    let mut current = RUNNING.load(Ordering::Relaxed);
    while current < max {
        match RUNNING.compare_exchange(current, current + 1, Ordering::AcqRel, Ordering::Relaxed) {
            Ok(_) => return Some(Permit),
            Err(next) => current = next,
        }
    }
    None
}

impl Drop for Permit {
    fn drop(&mut self) {
        RUNNING.fetch_sub(1, Ordering::AcqRel);
    }
}

fn max_jobs() -> usize {
    std::env::var("CODETETHER_RLM_BG_MAX")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
        .max(1)
}
