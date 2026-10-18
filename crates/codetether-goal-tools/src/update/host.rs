//! Native verifier construction with existing storage and audit boundaries.
use super::Args;
use codetether_goal_verification::VerifierAgent;
/// Host operations for a verified terminal goal transition.
/// # Implementors
/// Native runtimes combine their existing goal storage and audit operations
/// with the provider-backed verifier type already used by the application.
pub trait Host: crate::Host + codetether_goal_verification::audit::Host {
    /// Concrete verifier retaining its native execution implementation.
    type Verifier: VerifierAgent;
    /// Construct the verifier using the injected model and workspace context.
    fn verifier(args: &Args) -> Self::Verifier;
}
