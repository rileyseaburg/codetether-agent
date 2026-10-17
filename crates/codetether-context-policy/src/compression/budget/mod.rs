//! Trigger and retry policy for context compression.
//!
//! [check_budget] chooses a trigger from measured request cost and history size.
//! [attempt_fits] applies the history-length condition during the keep-last
//! schedule in [KEEP_LAST_CANDIDATES]. The host owns estimation and execution.
mod attempt;
mod trigger;
pub use attempt::attempt_fits;
pub use trigger::{BudgetCheck, check_budget};

/// Progressively smaller retained-tail sizes tried by the compression cascade.
pub const KEEP_LAST_CANDIDATES: [usize; 6] = [16, 12, 8, 6, 3, 1];
