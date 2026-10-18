//! Application-bound swarm executor construction and service access.
//!
//! [SwarmExecutor] owns its [crate::ExecutorState]. [ExecutorHost] supplies the
//! concrete coordinator type and connects execution to the application's stages.
use codetether_swarm_cache::SwarmCache;
use codetether_swarm_core::SwarmConfig;
use std::marker::PhantomData;
mod accessors;
mod cache_api;
mod configure;
mod configure_events;
mod execute_api;
mod host;
mod inline;
pub use host::ExecutorHost;
/// Swarm executor with application-supplied coordinator and stage bindings.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_swarm_runtime::ExecutorHost>() {
/// let executor = codetether_swarm_runtime::SwarmExecutor::<H>::new(Default::default());
/// assert!(executor.control().is_none());
/// # }
/// ```
pub struct SwarmExecutor<H: ExecutorHost> {
    /// Runtime services used by the application's stage adapters.
    pub inner: crate::ExecutorState<H::Agent, SwarmCache>,
    application: PhantomData<fn() -> H>,
}
impl<H: ExecutorHost> From<SwarmConfig> for SwarmExecutor<H> {
    fn from(config: SwarmConfig) -> Self {
        Self::new(config)
    }
}
