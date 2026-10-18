//! Swarm Execute Tool - Parallel task execution across multiple agents
//!
//! This tool enables LLM-driven parallel execution of tasks across multiple
//! sub-agents in a swarm pattern, with configurable concurrency and aggregation.
use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub(crate) mod agent_prompt;
pub(crate) mod agent_registry;
mod default_impl;
mod provider_setup;
mod runtime_host;
use codetether_swarm_tool::schema;
pub(crate) use codetether_swarm_tool::support;
use codetether_swarm_tool::task_input;
use codetether_swarm_tool::task_result;
pub(crate) mod tui_bridge;

pub struct SwarmExecuteTool;

impl SwarmExecuteTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for SwarmExecuteTool {
    fn id(&self) -> &str {
        "swarm_execute"
    }

    fn name(&self) -> &str {
        "Swarm Execute"
    }

    fn description(&self) -> &str {
        "Run independent tasks in parallel. Pass tasks as instruction strings, for example \
         {\"tasks\":[\"Inspect the API\",\"Run focused tests\"]}. Use task objects only when \
         names, IDs, or specialties are needed; orchestration settings are optional."
    }

    fn parameters(&self) -> Value {
        schema::parameters()
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        codetether_swarm_tool::runtime::execute::<runtime_host::Native>(params).await
    }
}
