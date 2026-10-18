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

use crate::{Agent, Host};
use anyhow::Result;
use async_trait::async_trait;
use codetether_swarm_core::{Actor, ActorStatus};

#[async_trait]
impl<H: Host> Actor for Agent<H> {
    fn actor_id(&self) -> &str {
        &self.info.name
    }
    fn actor_status(&self) -> ActorStatus {
        ActorStatus::Ready
    }

    async fn initialize(&mut self) -> Result<()> {
        tracing::info!(target: "codetether_agent::agent::swarm::actor_impl",agent = %self.info.name, "Agent initialized for swarm participation");
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        tracing::info!(target: "codetether_agent::agent::swarm::actor_impl",agent = %self.info.name, "Agent shutting down");
        Ok(())
    }
}
