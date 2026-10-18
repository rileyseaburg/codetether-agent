//! Ralph agent composition using shared prompts, registries and agent execution.
//!
//! [`ApplicationAgent`] implements [`crate::Host`] for an application tool
//! registry and a compatible [`codetether_swarm_agent::Host`]. Use it with
//! [`crate::direct`], [`crate::parallel`] or [`crate::conflict`].
use std::marker::PhantomData;
mod execute;
mod host;

/// Ralph agent binding a registry application to its existing agent policy.
/// # Examples
/// ```no_run
/// # fn example<A, H>() where A: codetether_application_registry::Application,
/// # H: codetether_swarm_agent::Host<Registry = codetether_application_registry::ToolRegistry<A>> {
/// type Bound<A, H> = codetether_ralph_agent::application::ApplicationAgent<A, H>;
/// let _prompt = <Bound<A, H> as codetether_ralph_agent::Host>::system_prompt(std::path::Path::new("."));
/// # }
/// ```
pub struct ApplicationAgent<A, H>(PhantomData<fn() -> (A, H)>);
