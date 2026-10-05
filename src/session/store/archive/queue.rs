//! Coalescing upload wakeups, including arrivals during the worker's final drain.
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
fn state() -> &'static Mutex<HashMap<String, bool>> {
    static STATE: OnceLock<Mutex<HashMap<String, bool>>> = OnceLock::new();
    STATE.get_or_init(Default::default)
}
pub(super) fn start(id: &str) -> bool {
    let mut jobs = state().lock().unwrap();
    if let Some(pending) = jobs.get_mut(id) {
        *pending = true;
        false
    } else {
        jobs.insert(id.into(), false);
        true
    }
}
pub(super) fn again(id: &str) -> bool {
    let mut jobs = state().lock().unwrap();
    if jobs.get(id) == Some(&true) {
        jobs.insert(id.into(), false);
        true
    } else {
        jobs.remove(id);
        false
    }
}
pub(super) fn remove(id: &str) {
    state().lock().unwrap().remove(id);
}
#[test]
fn arrivals_during_drain_are_not_lost() {
    let id = uuid::Uuid::new_v4().to_string();
    assert!(start(&id));
    assert!(!start(&id));
    assert!(again(&id));
    assert!(!again(&id));
    assert!(start(&id));
    remove(&id);
}
