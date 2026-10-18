//! Reviewer integration with application registries and the shared swarm agent.
//!
//! [`Reviewer`] binds direct review and background dispatch to an [`Application`].
//! [`read_only_tools`] builds the same restricted catalog for both entry points.
use std::marker::PhantomData;
mod dispatch;
mod host;
mod policy;
mod tools;
pub use policy::Application;
pub use tools::read_only_tools;

/// Static reviewer binding to provider, tool and agent application choices.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_approval_review::application::Application>() {
/// type Bound<H> = codetether_approval_review::application::Reviewer<H>;
/// let _tools = <Bound<H> as codetether_approval_review::Host>::tools();
/// # }
/// ```
pub struct Reviewer<H>(PhantomData<fn() -> H>);
