//! Bounded, process-local HWND logical state storage.
use super::{State, Target};
use anyhow::{Result, anyhow, ensure};
use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard, OnceLock},
};

pub(super) struct Entry {
    pub target: Target,
    pub state: State,
}
type States = HashMap<i64, Entry>;
pub(super) fn lock() -> Result<MutexGuard<'static, States>> {
    static STATES: OnceLock<Mutex<States>> = OnceLock::new();
    STATES
        .get_or_init(Mutex::default)
        .lock()
        .map_err(|_| anyhow!("Shadow state lock poisoned; no physical fallback"))
}
pub(super) fn entry(states: &mut States, target: Target) -> Result<&mut Entry> {
    ensure!(
        states.contains_key(&target.hwnd) || states.len() < 1024,
        "Shadow state capacity reached; stop tracked HWNDs first"
    );
    let entry = states.entry(target.hwnd).or_insert(Entry {
        target,
        state: State::default(),
    });
    if entry.target != target {
        // Never deliver releases for an old owner to a newly recycled HWND.
        *entry = Entry {
            target,
            state: State::default(),
        };
    }
    Ok(entry)
}
