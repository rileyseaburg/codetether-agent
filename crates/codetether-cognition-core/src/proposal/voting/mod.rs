//! Quorum, veto, approver and deadline rules for proposal resolution.
//!
//! [resolve] updates one proposal and returns optional attention. The caller
//! decides which proposals are pending and owns collection locking.
mod attention;
mod deadline;
mod resolve;
mod tally;
pub use attention::{missing_approvers, vote_timeout};
pub use deadline::{past_deadline, reject_on_deadline};
pub use resolve::resolve;
pub use tally::{approved_by_majority, is_vetoed, required_approvers_met};

mod sweep;
/// Resolve pending proposals in shared stores.
pub use sweep::resolve_pending;
