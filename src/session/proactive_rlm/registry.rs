//! Process-wide runtime registry and latest-snapshot coalescing.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use super::types::{Runtime, Snapshot, State};

pub(super) static STATES: OnceLock<Mutex<HashMap<String, State>>> = OnceLock::new();

pub(super) fn register(session_id: String, runtime: Runtime) {
    let Ok(mut states) = states().lock() else {
        return;
    };
    states
        .entry(session_id)
        .and_modify(|state| state.runtime = runtime.clone())
        .or_insert_with(|| State::new(runtime));
}

pub(super) fn enqueue(session_id: &str, build: impl FnOnce(Runtime, u64) -> Snapshot) -> bool {
    let Ok(mut states) = states().lock() else {
        return false;
    };
    let Some(state) = states.get_mut(session_id) else {
        return false;
    };
    let runtime = state.runtime.clone();
    state.slot.enqueue(|generation| build(runtime, generation))
}

pub(super) fn take(session_id: &str) -> Option<Snapshot> {
    states().lock().ok()?.get_mut(session_id)?.slot.take()
}

pub(super) fn settle(session_id: &str) -> bool {
    let Ok(mut states) = states().lock() else {
        return true;
    };
    states
        .get_mut(session_id)
        .is_none_or(|state| state.slot.settle())
}

fn states() -> &'static Mutex<HashMap<String, State>> {
    STATES.get_or_init(|| Mutex::new(HashMap::new()))
}
