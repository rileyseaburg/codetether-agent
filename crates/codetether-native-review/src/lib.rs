//! Native approval-review runtime and goal-aware application binding.
//!
//! [`runtime`] owns the single installed reviewer configuration and provider
//! registry. [`Application`] reads durable goal journals and selects native
//! providers, while its type parameters select the tool catalog and agent loop.
//! [`Reviewer`] exposes the shared review and background-dispatch contracts.
use std::marker::PhantomData;
mod application;
pub mod runtime;

/// Native reviewer application using tool catalog `T` and agent loop `A`.
///
/// The marker is selected as a type and does not need construction.
///
/// # Examples
/// ```no_run
/// # fn example<T, A>() where T: codetether_application_registry::Application, A: codetether_swarm_agent::Host<Registry = codetether_application_registry::ToolRegistry<T>> {
/// type Bound<T, A> = codetether_native_review::Application<T, A>;
/// let _runtime = <Bound<T, A> as codetether_approval_review::application::Application>::enabled();
/// # }
/// ```
pub struct Application<T, A>(PhantomData<fn() -> (T, A)>);

/// Shared reviewer specialized to the native runtime and supplied app types.
pub type Reviewer<T, A> = codetether_approval_review::application::Reviewer<Application<T, A>>;

mod binding;

/// Shared verdict types, parser and review prompts.
pub use codetether_approval_core::review::{
    ReviewOutcome, ReviewSubject, ReviewVerdict, parse, prompt,
};

/// Dependencies used by the application declaration.
#[doc(hidden)]
pub mod __private {
    pub use codetether_application_registry::ToolRegistry;
    pub use codetether_approval_queue::ApprovalSnapshot;
    pub use codetether_approval_review::{application, dispatch, review};
    pub use codetether_config::ReviewConfig;
    pub use codetether_native_providers::ProviderRegistry;
}
