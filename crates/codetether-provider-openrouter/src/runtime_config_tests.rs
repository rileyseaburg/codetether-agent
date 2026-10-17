use super::{set_thinking_level, thinking_level};
use std::sync::{Mutex, MutexGuard, OnceLock};

/// Serializes these tests: they mutate one process-wide override, so running
/// them concurrently makes each one observe the other's writes.
fn guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn override_round_trips_without_environment_mutation() {
    let _lock = guard();
    let previous = thinking_level();
    set_thinking_level(Some("xhigh".to_string()));
    assert_eq!(thinking_level().as_deref(), Some("xhigh"));
    set_thinking_level(previous);
}

#[test]
fn clearing_the_override_restores_the_unset_state() {
    let _lock = guard();
    let previous = thinking_level();
    set_thinking_level(Some("low".to_string()));
    set_thinking_level(None);
    assert!(thinking_level().is_none());
    set_thinking_level(previous);
}
