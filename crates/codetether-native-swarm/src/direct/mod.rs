//! Native direct-swarm tool services and shared worker registries.
//!
//! [Host] binds provider discovery, TUI observation and the native agent loop.
//! [registry] retains standard and result-sharing registry composition for the
//! application's existing tool catalog T.
use codetether_application_registry::{Application, ToolRegistry};
use std::marker::PhantomData;
mod host;
mod providers;
pub mod registry;
mod worker;
/// Native direct-swarm runtime for application tool catalog T.
/// # Examples
/// The application selects this marker as the shared swarm tool's host.
/// ```
/// let _: Option<codetether_native_swarm::direct::Host<()>> = None;
/// ```
pub struct Host<T>(PhantomData<fn() -> T>);
