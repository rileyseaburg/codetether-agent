//! Thread-local step-budget outcome tracking for prompt loops.
//!
//! [begin] marks whether a run has an explicit budget; [clear_budget] marks
//! successful completion within it. [was_budget_exhausted] reads the remaining
//! flag. State belongs to the executing OS thread, matching the existing loop.
mod lifecycle;
mod outcome;
mod state;
pub use lifecycle::{begin, clear_budget};
pub use outcome::was_budget_exhausted;
