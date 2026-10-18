//! Tool identity, schema and execution through the shared swarm runtime.
use crate::{SwarmExecuteTool, runtime::Host, schema};
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;

#[async_trait]
impl<H: Host> Tool for SwarmExecuteTool<H> {
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
        crate::runtime::execute::<H>(params).await
    }
}
