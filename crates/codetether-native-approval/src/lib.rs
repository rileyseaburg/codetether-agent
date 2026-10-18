//! Native approval records, live requests and shared grant access.
//!
//! [ApprovalStore] persists decisions; [live] publishes live requests.
//! [session_grants] and [session_command_grants] expose the same process-wide
//! grant stores as approval-core. The test-support feature exposes one shared
//! environment lock for integration tests across dependent crates.
#[cfg(any(test, feature = "test-support"))]
use codetether_config as config;
mod approval;
pub use approval::*;

mod child;
pub use child::approve_autonomous_child;
