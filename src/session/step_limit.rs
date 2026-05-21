//! Step-limit outcome tracking for non-interactive resumable runs.
//!
//! Uses a thread-local flag so that concurrent sessions in separate threads
//! (swarm, relay, server) do not race on a single global. Each CLI invocation
//! or task-fiber gets its own independent flag.

use std::cell::Cell;

thread_local! {
    static EXHAUSTED: Cell<bool> = const { Cell::new(false) };
}

pub(crate) fn start_step_limited_run() {
    EXHAUSTED.with(|c| c.set(true));
}

pub(crate) fn mark_completed() {
    EXHAUSTED.with(|c| c.set(false));
}

pub fn was_exhausted() -> bool {
    EXHAUSTED.with(|c| c.get())
}
