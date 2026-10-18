//! Verifier runtime composition using application providers and workspace tools.
//!
//! [`ApplicationHost`] connects the observed verifier runtime to provider cache,
//! scoped tools and a shared agent loop. Type parameters select the provider,
//! tool, agent and workspace implementations without native crate dependencies.
use std::marker::PhantomData;
mod execute;
mod host;
/// Verifier prompts using the shared direct-worker instruction contract.
pub mod prompt;

/// Static verifier binding for providers `P`, tools `T`, agent `A` and workspace `W`.
/// # Examples
/// ```no_run
/// # fn example<P, T, A, W>() where P: codetether_application_providers::Application, T: codetether_application_registry::Application, A: codetether_swarm_agent::Host<Registry = codetether_application_registry::ToolRegistry<T>>, W: codetether_workspace_tools::composition::Factory<Registry = codetether_application_registry::ToolRegistry<T>> {
/// let _name = std::any::type_name::<
///     codetether_goal_verification::runtime::application::ApplicationHost<P, T, A, W>
/// >();
/// # }
/// ```
pub struct ApplicationHost<P, T, A, W>(PhantomData<fn() -> (P, T, A, W)>);
