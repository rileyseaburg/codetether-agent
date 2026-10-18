//! Construction and dependency attachment for [`SwarmExecutor`].

use super::SwarmExecutor;
use crate::agent::Agent;
use crate::bus::AgentBus;
use crate::session::delegation::DelegationState;
use crate::swarm::{CacheConfig, SwarmCache, SwarmConfig, SwarmControl};
use anyhow::Result;
use std::sync::Arc;

impl SwarmExecutor {
    /// Creates an executor from a swarm configuration.
    pub fn new(config: SwarmConfig) -> Self {
        Self {
            inner: codetether_swarm_runtime::ExecutorState::new(config),
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
    pub fn with_coordinator_agent(mut self, agent: Arc<tokio::sync::Mutex<Agent>>) -> Self {
        self.inner = self.inner.with_coordinator_agent(agent);
        self
    }
}
