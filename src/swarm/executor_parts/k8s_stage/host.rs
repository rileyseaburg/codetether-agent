//! Existing executor configuration, control, cache and event bindings.
use super::SwarmExecutor;
use crate::swarm::{SwarmCache, SwarmConfig, SwarmControl};
use codetether_swarm_core::SwarmEvent;
use std::sync::Arc;
use tokio::sync::Mutex;
pub(super) struct Native<'a>(pub &'a SwarmExecutor);
impl codetether_swarm_k8s::Host for Native<'_> {
    type Cache = SwarmCache;
    fn config(&self) -> &SwarmConfig {
        &self.0.inner.config
    }
    fn control(&self) -> Option<&SwarmControl> {
        self.0.control()
    }
    fn cache(&self) -> Option<&Arc<Mutex<SwarmCache>>> {
        self.0.inner.cache.as_ref()
    }
    fn try_send_event(&self, event: SwarmEvent) {
        self.0.try_send_event(event);
    }
}
