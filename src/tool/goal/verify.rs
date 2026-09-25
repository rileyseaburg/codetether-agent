//! # Independent goal verification
//!
//! Terminal goal transitions (`complete` and `blocked`) are decided by a
//! second LLM agent running in its own thread, separate from the agent that
//! did the work. `update_goal` hands that verifier the persisted goal and the
//! worker's evidence; the goal only changes state when the verifier returns
//! [`Verdict::Pass`]. Any other outcome keeps the goal active and returns the
//! verifier's findings to the worker.
//!
//! ## Key types
//!
//! - [`VerificationRequest`] — the goal requirements plus the worker's claim.
//! - [`VerifierAgent`] — anything that can review a request and report back.
//! - [`LlmVerifier`] — the production verifier backed by an agent loop.
//! - [`Verdict`] — the verifier's decision.
//!
//! ## Usage
//!
//! ```rust
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! use codetether_agent::session::tasks::GoalStatus;
//! use codetether_agent::tool::goal::verify::{
//!     Verdict, VerificationRequest, VerifierAgent, verify_transition,
//! };
//!
//! struct ApprovingVerifier;
//!
//! #[async_trait::async_trait]
//! impl VerifierAgent for ApprovingVerifier {
//!     async fn review(&self, _: &VerificationRequest) -> anyhow::Result<String> {
//!         Ok("PASS — flag exists — src/cli.rs:12\nVERDICT: PASS".into())
//!     }
//! }
//!
//! let request = VerificationRequest {
//!     objective: "Add a --json flag".into(),
//!     success_criteria: vec![],
//!     forbidden: vec![],
//!     claimed: GoalStatus::Complete,
//!     evidence: "src/cli.rs:12 defines --json".into(),
//! };
//! assert_eq!(verify_transition(&ApprovingVerifier, &request).await, Verdict::Pass);
//! # });
//! ```

#[path = "verify_agent.rs"]
mod agent;
#[path = "verify_charter.rs"]
mod charter;
#[path = "verify_gate.rs"]
mod gate;
#[path = "verify_llm.rs"]
mod llm;
#[path = "verify_model.rs"]
mod model;
#[path = "verify_prompt.rs"]
mod prompt;
#[path = "verify_request.rs"]
mod request;
#[path = "verify_selection.rs"]
mod selection;
#[path = "verify_verdict.rs"]
mod verdict;

pub use agent::VerifierAgent;
pub use charter::charter;
pub use gate::verify_transition;
pub use llm::LlmVerifier;
pub use model::{VERIFIER_MODEL_ENV, resolve_verifier_model, select_verifier_model};
pub use prompt::{system_prompt, user_prompt};
pub use request::VerificationRequest;
pub use selection::{selected_verifier_model, set_verifier_model};
pub use verdict::Verdict;
