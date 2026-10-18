//! Thread-local budget-active flag shared by all step-limit operations.
use std::cell::Cell;

thread_local! {
    pub(crate) static BUDGET_ACTIVE: Cell<bool> = const { Cell::new(false) };
}
