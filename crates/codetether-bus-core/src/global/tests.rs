//! Checks stable access to the shared process bus.
use super::*;

#[test]
fn global_is_none_by_default_or_already_set() {
    // Cannot reliably test set_global in a unit test because OnceLock
    // persists across tests in the same binary. We only assert that
    // `global()` either returns None or returns Some consistently.
    let first = global().map(|b| Arc::as_ptr(&b) as usize);
    let second = global().map(|b| Arc::as_ptr(&b) as usize);
    assert_eq!(first, second);
}
