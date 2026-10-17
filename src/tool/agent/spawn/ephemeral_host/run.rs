//! Invoke the native agent loop with the original one-shot limits.
use super::ApplicationHost;
use crate::swarm::executor::{AgentLoopExit, run_agent_loop};
use crate::tool::ToolRegistry;
use anyhow::Result;
use async_trait::async_trait;
use codetether_agent_spawn::{EphemeralHost, EphemeralRun};
#[async_trait]
impl EphemeralHost for ApplicationHost {
    async fn execute(
        run: EphemeralRun<ToolRegistry>,
        instructions: &str,
        runtime_id: String,
    ) -> Result<(String, usize, usize, AgentLoopExit)> {
        run_agent_loop(
            run.provider,
            &run.model,
            &run.prompt,
            instructions,
            run.registry.definitions(),
            run.registry,
            crate::session::DEFAULT_MAX_STEPS,
            300,
            None,
            runtime_id,
            None,
            Some(run.workspace),
        )
        .await
    }
}
