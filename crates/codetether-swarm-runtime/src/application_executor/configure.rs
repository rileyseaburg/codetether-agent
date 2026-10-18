//! Construction and dependency attachment for [`SwarmExecutor`].

use super::{ExecutorHost, SwarmExecutor};
use anyhow::Result;
use codetether_bus_core::AgentBus;
use codetether_context_policy::delegation::DelegationState;
use codetether_session_control::ControlHandle as SwarmControl;
use codetether_swarm_cache::{CacheConfig, SwarmCache};
use codetether_swarm_core::SwarmConfig;
use std::sync::Arc;

impl<H: ExecutorHost> SwarmExecutor<H> {
    /// Creates an executor from a swarm configuration.
    pub fn new(config: SwarmConfig) -> Self {
        Self {
            application: super::PhantomData,
            inner: crate::ExecutorState::new(config),
        }
    }

    /// Creates an executor with a persistent result cache.
    ///
    /// # Errors
    ///
    /// Returns an error when the configured cache cannot be initialized.
    pub async fn with_cache(config: SwarmConfig, cache_config: CacheConfig) -> Result<Self> {
        let cache = SwarmCache::new(cache_config).await?;
        Ok(Self::new(config).with_cache_instance(Arc::new(tokio::sync::Mutex::new(cache))))
    }

    /// Attaches a pre-initialized cache.
    pub fn with_cache_instance(mut self, cache: Arc<tokio::sync::Mutex<SwarmCache>>) -> Self {
        self.inner = self.inner.with_cache_instance(cache);
        self
    }

    /// Attaches an inter-agent message bus.
    pub fn with_bus(mut self, bus: Arc<AgentBus>) -> Self {
        self.inner = self.inner.with_bus(bus);
        self
    }

    /// Attaches runtime cancellation and pause control.
    pub fn with_control(mut self, control: SwarmControl) -> Self {
        self.inner = self.inner.with_control(control);
        self
    }

    /// Attaches adaptive provider-delegation state.
    pub fn with_delegation(mut self, state: DelegationState) -> Self {
        self.inner = self.inner.with_delegation(state);
        self
    }

    /// Attaches an agent used for swarm-level coordination.
    pub fn with_coordinator_agent(mut self, agent: Arc<tokio::sync::Mutex<H::Agent>>) -> Self {
        self.inner = self.inner.with_coordinator_agent(agent);
        self
    }
}
