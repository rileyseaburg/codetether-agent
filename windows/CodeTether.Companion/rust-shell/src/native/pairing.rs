use super::{background, pair_controls, pair_task::PairTask, state::State};
use crate::relay;
use std::sync::Arc;
use zeroize::Zeroizing;

pub(super) fn begin(state: &mut State) {
    if state.pair_task.is_some() {
        return;
    }
    let Some(c) = &state.controls else {
        return;
    };
    let Some(code) = relay::normalize(&pair_controls::take_code(c.code)) else {
        state.status = "Sharing OFF — enter the 12-character iPhone pairing code.";
        return;
    };
    let code = Zeroizing::new(code);
    background::forget(state);
    state.status = "Sharing OFF — pairing with the relay…";
    match std::thread::Builder::new()
        .name("companion-pair".into())
        .spawn(move || relay::pair(&code))
    {
        Ok(thread) => {
            state.pair_task = Some(PairTask {
                valid: true,
                thread,
            })
        }
        Err(_) => state.status = "Sharing OFF — cannot start pairing worker.",
    }
    pair_controls::busy(state, state.pair_task.is_some());
}

pub(super) fn finish(state: &mut State) {
    if !state
        .pair_task
        .as_ref()
        .is_some_and(|task| task.thread.is_finished())
    {
        return;
    }
    let task = state.pair_task.take().unwrap();
    let outcome = task
        .thread
        .join()
        .unwrap_or_else(|_| Err("Pairing worker failed.".into()));
    pair_controls::busy(state, false);
    if !task.valid {
        return;
    }
    match outcome {
        Ok(device) => {
            state.device = Some(Arc::new(device));
            state.status =
                "Paired — select the shared monitor; iPhone requests run in the background.";
        }
        Err(message) => {
            state.pair_error = message;
            state.status = "Sharing OFF — pairing failed.";
        }
    }
}
