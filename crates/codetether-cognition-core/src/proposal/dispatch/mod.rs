//! Approval registration and verified-proposal receipt recording.
//!
//! [approve_proposal] preserves the Critical/Verified gate. [execute_verified]
//! records receipts and events, waiting for approval on Critical proposals.
mod approval;
mod context;
mod execute;
mod receipt;
pub use approval::approve_proposal;
pub use context::ExecuteCtx;
pub use execute::execute_verified;
pub use receipt::{executed_event, receipt_for};
