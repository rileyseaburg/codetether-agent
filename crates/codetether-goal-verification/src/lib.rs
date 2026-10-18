//! Goal verification protocol and durable verdict accounting.
//!
//! [VerificationRequest] copies the persisted requirements for a [VerifierAgent].
//! [verify_with_identity] evaluates the report with its captured execution
//! identity. [VerdictRecord], [journal] and [attempt_cap] retain audit history.
//! Provider execution and session-path resolution belong to callers.
mod agent;
pub mod attempt_cap;
mod charter;
mod execution;
mod gate;
pub mod journal;
mod request;
mod user_prompt;
mod verdict_record;
pub use agent::VerifierAgent;
pub use charter::charter;
pub use codetether_verifier_state::Verdict;
pub use execution::ReviewExecution;
pub use gate::{verify_transition, verify_with_identity};
pub use request::VerificationRequest;
pub use user_prompt::user_prompt;
pub use verdict_record::VerdictRecord;

/// Audited verification and capped-rejection handling through native storage.
pub mod audit;

/// Observed provider-backed verifier execution.
pub mod runtime;
mod system_prompt;
pub use system_prompt::system_prompt;
