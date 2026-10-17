//! Native one-shot agent-loop invocation.
use super::{EphemeralPolicy, EphemeralResources};
use crate::EphemeralRun;
use anyhow::Result;
use async_trait::async_trait;
use codetether_swarm_core::agent_retry::AgentLoopExit;
/// Execute the existing agent loop after shared ephemeral setup.
/// # Implementors
/// Implement both setup traits, then forward the prepared run to the native loop.
#[async_trait]
pub trait EphemeralHost: EphemeralResources + EphemeralPolicy {
    /// Run one ephemeral child, preserving native limits and result/error semantics.
    async fn execute(
        run: EphemeralRun<Self::Registry>,
        instructions: &str,
        runtime_id: String,
    ) -> Result<(String, usize, usize, AgentLoopExit)>;
}
