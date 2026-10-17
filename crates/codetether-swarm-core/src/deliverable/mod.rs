//! Delegated deliverable status and verification evidence policy.
//!
//! [VerificationContract] infers verification requirements, [verify_output]
//! calibrates unsupported preservation claims, and [deliverable_error] detects
//! terminal non-completion. [status_contract] supplies the existing prompt text.
mod deliverable_contract;
mod deliverable_status;
mod verification_contract;
mod verification_language;
mod verification_output;
pub use deliverable_contract::render as status_contract;
pub use deliverable_status::error as deliverable_error;
pub use verification_contract::VerificationContract;
pub use verification_output::for_instruction as verify_output;
