//! Invoke the native agent loop with the original one-shot limits.
use super::{Application, Ephemeral, ToolRegistry};
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_spawn::{EphemeralHost, EphemeralRun};
use codetether_swarm_core::agent_retry::AgentLoopExit;
#[async_trait]
impl<T: Application, const MAX_STEPS: usize> EphemeralHost for Ephemeral<T, MAX_STEPS> {
    async fn execute(
        run: EphemeralRun<ToolRegistry<T>>,
        instructions: &str,
        runtime_id: String,
    ) -> Result<(String, usize, usize, AgentLoopExit)> {
        codetether_swarm_agent::run_agent_loop::<codetether_native_swarm::AgentHost<T>>(
            run.provider,
            &run.model,
            &run.prompt,
            instructions,
            run.registry.definitions(),
            run.registry,
            MAX_STEPS,
            300,
            None,
            runtime_id,
            None,
            Some(run.workspace),
        )
        .await
    }
}
