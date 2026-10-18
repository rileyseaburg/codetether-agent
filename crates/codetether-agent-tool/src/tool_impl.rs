//! Shared Tool implementation and parse/hydrate/dispatch sequence.
use crate::{AgentTool, Host};
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;

#[async_trait]
impl<H: Host> Tool for AgentTool<H> {
    fn id(&self) -> &str {
        "agent"
    }

    fn name(&self) -> &str {
        "Sub-Agent"
    }

    fn description(&self) -> &str {
        "First-party collaboration with local agents and authenticated LAN peers. Actions include spawn, message, list, status, interrupt, close, resume, and kill. Mux TUI sessions cannot be controlled through this tool."
    }

    fn parameters(&self) -> Value {
        crate::schema::agent_tool_parameters()
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let parsed: codetether_agent_input::Params =
            serde_json::from_value(params).context("Invalid params")?;
        H::hydrate(parsed.parent_session_id.as_deref()).await?;
        crate::dispatch::execute::<H>(&parsed).await
    }
}
