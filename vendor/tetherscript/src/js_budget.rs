//! Execution budget for the dependency-free JavaScript engine.

use std::cell::Cell;

thread_local! {
    static REMAINING: Cell<Option<u64>> = const { Cell::new(None) };
}

pub(crate) fn with<R>(budget: u64, f: impl FnOnce() -> R) -> R {
    let previous = REMAINING.with(|remaining| remaining.replace(Some(budget)));
    let result = f();
    REMAINING.with(|remaining| remaining.set(previous));
    result
}

pub(crate) fn tick() -> Result<(), String> {
    let exhausted = REMAINING.with(|remaining| match remaining.get() {
        None => false,
        Some(0) => true,
        Some(count) => {
            remaining.set(Some(count - 1));
            false
        }
    });
    if exhausted {
        Err("js_eval: execution budget exhausted (possible infinite loop)".into())
    } else {
        Ok(())
    }
}
