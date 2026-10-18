//! Native state, audited transitions and verdict journals for goal tools.
//!
//! [Host] delegates goal reads and edits to native runtime/control services.
//! [VerifierFactory] keeps the application's verifier and public construction
//! fields intact while the native host selects its model/workspace inputs.
use std::{marker::PhantomData, path::PathBuf};
mod audit;
mod host;
pub mod verdict_log;
/// Application verifier construction for terminal goal transitions.
/// # Implementors
/// Implement on the existing independent verifier, retaining both inputs.
pub trait VerifierFactory: codetether_goal_verification::VerifierAgent + Sized {
    /// Construct a verifier for the worker model and resolved workspace.
    fn create(worker_model: Option<String>, workspace: PathBuf) -> Self;
}
/// Native goal-tool host selecting verifier type F.
/// # Examples
/// ```
/// let _: Option<codetether_native_goal_tools::Host<()>> = None;
/// ```
pub struct Host<F>(PhantomData<fn() -> F>);

mod declaration;
/// Independent native verifier runtime and review dispatch.
pub mod verification;
/// Types used by the application-local declaration macro.
#[doc(hidden)]
pub mod __private {
    pub use anyhow::Result;
    pub use async_trait::async_trait;
    pub use codetether_goal_verification::{ReviewExecution, VerificationRequest, VerifierAgent};
}
