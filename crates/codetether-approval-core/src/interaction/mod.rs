//! Approval interaction records and command-prefix proposals.
//!
//! Build a [LiveApprovalRequest] and present its [ReviewDecision] choices in a
//! client. Applications deliver requests and apply grants; these types only
//! describe that exchange. [prefix::from_args] parses a proposed amendment.
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
