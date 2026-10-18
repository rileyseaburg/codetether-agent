//! Approval interaction records and command-prefix proposals.
//!
//! Build a [LiveApprovalRequest] and present its [ReviewDecision] choices in a
//! client. [handoff::request] owns pending-request registration and cleanup;
//! applications deliver requests and apply grants. [prefix::from_args] parses
//! a proposed amendment.
mod amendment;
mod decision;
pub mod prefix;
mod request;
mod request_arguments;
mod request_build;
mod review;
pub use amendment::ExecPolicyAmendment;
pub use decision::LiveApprovalDecision;
pub use request::LiveApprovalRequest;
pub use review::ReviewDecision;

mod decision_kind;
pub mod pending;
pub mod pending_query;
pub use decision_kind::ApprovalDecisionKind;

/// Request delivery, decision waiting, and cancellation-safe cleanup.
pub mod handoff;
