//! Background authorization comes from pairing plus local monitor selection.
use super::{state::State, worker::Worker};
use std::sync::atomic::Ordering;

#[derive(Default)]
pub(super) struct Background {
    pub(super) paused: bool,
    pub(super) worker: Option<Worker>,
    pub(super) suspended: bool,
    pub(super) retry_at: Option<std::time::Instant>,
}
pub(super) fn cancel(state: &State) {
    state.interrupt.store(true, Ordering::Release);
    if let Some(worker) = &state.background.worker {
        worker.stop.cancel();
    }
}
pub(super) fn forget(state: &mut State) {
    cancel(state);
    state.device = None;
    if let Some(task) = &mut state.pair_task {
        task.valid = false;
    }
}
pub(super) fn tick(state: &mut State) {
    super::work_reap::finish(state);
    if state.device.as_ref().is_some_and(|device| device.expired()) {
        forget(state);
        state.status = "Sharing OFF — session expired; pair again.";
    }
    if state.background.paused || !state.desktop_available {
        return;
    }
    if state
        .background
        .retry_at
        .is_some_and(|at| std::time::Instant::now() < at)
    {
        return;
    }
    state.background.retry_at = None;
    if state.background.worker.is_none() {
        if let (Some(device), Some(monitor)) = (&state.device, &state.selected) {
            match Worker::spawn(device.clone(), monitor.clone(), state.interrupt.clone()) {
                Ok(job) => state.background.worker = Some(job),
                Err(_) => {
                    state.background.paused = true;
                    state.status = "Sharing OFF — cannot start request worker.";
                }
            }
        }
    }
    if let Some(job) = &state.background.worker {
        if !job.stop.token.is_cancelled() {
            state.status = job.status();
        }
    }
}
