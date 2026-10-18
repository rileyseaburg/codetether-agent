//! Existing executor configuration, control, cache and event bindings.
use crate::{ExecutorHost, SwarmExecutor};
use codetether_session_control::ControlHandle as SwarmControl;
use codetether_swarm_cache::SwarmCache;
use codetether_swarm_core::SwarmConfig;
use codetether_swarm_core::SwarmEvent;
use std::sync::Arc;
use tokio::sync::Mutex;
impl<H: ExecutorHost> codetether_swarm_k8s::Host for SwarmExecutor<H> {
    type Cache = SwarmCache;
    fn config(&self) -> &SwarmConfig {
        &self.inner.config
    }
    fn control(&self) -> Option<&SwarmControl> {
        SwarmExecutor::control(self)
    }
    fn cache(&self) -> Option<&Arc<Mutex<SwarmCache>>> {
        self.inner.cache.as_ref()
    }
    fn try_send_event(&self, event: SwarmEvent) {
        SwarmExecutor::try_send_event(self, event);
    }
}
