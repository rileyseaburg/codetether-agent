//! First-class approval requests and receipts.
//!
//! The module stores approval requests as append-only JSONL events and
//! verifies approved receipts against a requested tool/action/resource tuple.
//! The ledger, interaction records and process-local grants live in
//! `codetether-approval-core`; this adapter delivers requests to live sessions.

use codetether_approval_core::interaction as decision_kind;
use codetether_approval_core::interaction as exec_policy;
pub use codetether_approval_session as live;
pub mod session_command_grants;
pub mod session_grants;

#[cfg(test)]
pub(crate) use codetether_approval_core::ApprovalEvent;
pub use codetether_approval_core::{
    ApprovalDecision, ApprovalReceipt, ApprovalRequest, ApprovalStatus, ApprovalStore,
};
pub use decision_kind::ApprovalDecisionKind;
pub use exec_policy::{ExecPolicyAmendment, ReviewDecision};
pub use live::{LiveApprovalDecision, LiveApprovalRequest};

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub mod test_env;
