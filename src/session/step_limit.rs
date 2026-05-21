//! Step-limit outcome tracking for non-interactive resumable runs.

use std::sync::atomic::{AtomicBool, Ordering};

static EXHAUSTED: AtomicBool = AtomicBool::new(false);

pub(crate) fn start_step_limited_run() {
    EXHAUSTED.store(true, Ordering::SeqCst);
}

pub(crate) fn mark_completed() {
    EXHAUSTED.store(false, Ordering::SeqCst);
}

pub fn was_exhausted() -> bool {
    EXHAUSTED.load(Ordering::SeqCst)
}
