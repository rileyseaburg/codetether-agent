//! Shared thread-local step-limit outcome tracking.
pub use codetether_step_limit::was_budget_exhausted;
pub(crate) use codetether_step_limit::{begin, clear_budget};
