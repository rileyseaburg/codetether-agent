//! Native provider, policy and agent-loop services for ephemeral children.
use codetether_application_registry::{Application, ToolRegistry};
use std::marker::PhantomData;
mod policy;
mod resources;
mod run;
/// One-shot child runtime using catalog T and the caller's default step limit.
/// # Examples
/// ```
/// let _: Option<codetether_native_agent::Ephemeral<(), 50>> = None;
/// ```
pub struct Ephemeral<T, const MAX_STEPS: usize>(PhantomData<fn() -> T>);
