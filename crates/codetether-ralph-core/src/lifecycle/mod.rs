//! Resolve terminal Ralph status from final progress and iteration counts.
//!
//! finalize_status preserves explicit completion and classifies incomplete runs
//! as exhausted or failed, retaining the original reporting behavior.
mod at_limit;
mod exhausted;
mod finalize;
pub use finalize::finalize_status;
