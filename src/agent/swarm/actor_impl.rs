//! Swarm actor lifecycle for agents.
//!
//! This module provides the lightweight lifecycle hooks required by the swarm
//! runtime when it manages an `Agent` actor.
//!
//! # Examples
//!
//! ```ignore
//! let status = agent.actor_status();
//! ```

use crate::agent::Agent;
use crate::swarm::{Actor, ActorStatus};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
impl Actor for Agent {
    fn actor_id(&self) -> &str {
        &self.info.name
    }
    fn actor_status(&self) -> ActorStatus {
        ActorStatus::Ready
    }

    async fn initialize(&mut self) -> Result<()> {
        tracing::info!(agent = %self.info.name, "Agent initialized for swarm participation");
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        tracing::info!(agent = %self.info.name, "Agent shutting down");
        Ok(())
    }
}
