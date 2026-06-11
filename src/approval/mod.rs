//! First-class approval requests and receipts.
//!
//! The module stores approval requests as append-only JSONL events and
//! verifies approved receipts against a requested tool/action/resource tuple.

mod decision;
mod decision_kind;
mod event;
mod exec_policy;
pub mod live;
mod receipt;
mod request;
pub mod session_command_grants;
pub mod session_grants;
mod status;
mod store;
mod store_create;
mod store_decide;
mod store_events;
mod store_lookup;
mod store_verify;

pub use decision::ApprovalDecision;
pub use decision_kind::ApprovalDecisionKind;
pub(crate) use event::ApprovalEvent;
pub use exec_policy::{ExecPolicyAmendment, ReviewDecision};
pub use live::{LiveApprovalDecision, LiveApprovalRequest};
pub use receipt::ApprovalReceipt;
pub use request::ApprovalRequest;
pub use status::ApprovalStatus;
pub use store::ApprovalStore;

#[cfg(test)]
pub(crate) mod test_env {
    pub(crate) static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
}

#[cfg(test)]
mod tests;
