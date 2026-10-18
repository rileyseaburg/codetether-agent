//! Ralph runtime composition over application registries, agent policy and relay.
//!
//! [`ApplicationHost`] selects the shared agent, quality and Git integrations.
//! Its relay parameter retains the embedding application's session operations.
use std::marker::PhantomData;
mod host;
/// Runtime binding for a provider registry, tool identity, agent host and relay.
/// # Examples
/// ```no_run
/// # fn example<P, T, A, R>() where
/// # P: Send + Sync + 'static,
/// # T: codetether_application_registry::Application,
/// # A: codetether_ralph_agent::Host<Registry = codetether_application_registry::ToolRegistry<T>>,
/// # R: codetether_ralph_relay::Host<Registry = P> {
/// fn accepts_runtime<H: codetether_ralph_runtime::Host>() {}
/// accepts_runtime::<codetether_ralph_runtime::application::ApplicationHost<P, T, A, R>>();
/// # }
/// ```
pub struct ApplicationHost<P, T, A, R>(PhantomData<fn() -> (P, T, A, R)>);
