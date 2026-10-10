//! Reap only finished threads; stale sessions never change the current UI.
use super::{background, state::State, worker::Exit};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

pub(super) fn finish(state: &mut State) {
    if !state
        .background
        .worker
        .as_ref()
        .is_some_and(|job| job.join.is_finished())
    {
        return;
    }
    let job = state.background.worker.take().unwrap();
    let current = state
        .device
        .as_ref()
        .is_some_and(|d| Arc::ptr_eq(d, &job.device));
    let result = job.join.join().unwrap_or(Exit::Failed);
    if !current {
        return;
    }
    match result {
        Exit::Revoked => {
            background::forget(state);
            state.status = "Sharing OFF — session ended; pair again.";
        }
        Exit::Failed => {
            state.background.paused = true;
            state.status = "Sharing OFF — request failed; resume to retry.";
        }
        Exit::Suspended => {
            state.background.retry_at = Some(Instant::now() + Duration::from_secs(5));
            if !state.background.paused {
                state.status = "Sharing suspended — capture unavailable; retrying.";
            }
        }
        Exit::Cancelled => {}
    }
}
