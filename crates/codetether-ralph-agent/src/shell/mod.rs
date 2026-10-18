//! Policy-gated shell execution with distinct quality and verification outcomes.
//!
//! [`Quality`] returns failed output for a policy denial. [`Verification`]
//! returns the existing command-specific error. Both use the application's
//! runtime authorization before invoking the shared Bash tool.
use std::marker::PhantomData;
mod quality;
mod verification;
/// Shell executor returning denied commands as unsuccessful quality output.
/// # Examples
/// ```no_run
/// # fn example<A: codetether_application_registry::Application>() {
/// let _executor = codetether_ralph_agent::shell::Quality::<A>::default();
/// # }
/// ```
pub struct Quality<A>(PhantomData<fn() -> A>);
impl<A> Default for Quality<A> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
/// Shell executor returning denied commands as story verification errors.
/// # Examples
/// ```no_run
/// # fn example<A: codetether_application_registry::Application>() {
/// let _executor = codetether_ralph_agent::shell::Verification::<A>::new();
/// # }
/// ```
pub struct Verification<A>(PhantomData<fn() -> A>);
impl<A> Verification<A> {
    /// Construct a stateless verification executor bound to application policy.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}
impl<A> Default for Verification<A> {
    fn default() -> Self {
        Self::new()
    }
}
