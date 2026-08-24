//! Mux-only control plane exposed to manager agents.

#[path = "mux_control/args.rs"]
mod args;
#[path = "mux_control/execute.rs"]
mod execute;
#[path = "mux_control/lifecycle.rs"]
mod lifecycle;
#[path = "mux_control/lifecycle_safety.rs"]
mod lifecycle_safety;
#[path = "mux_control/operations.rs"]
mod operations;
#[path = "mux_control/schema.rs"]
mod schema;
#[path = "mux_control/workspace.rs"]
mod workspace;
#[path = "mux_control/unsafe_process_policy.rs"]
mod unsafe_process_policy;

use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub(super) struct MuxControlTool;

#[async_trait]
impl Tool for MuxControlTool {
    fn id(&self) -> &str {
        "mux_control"
    }
    fn name(&self) -> &str {
        "Mux Control"
    }
    fn description(&self) -> &str {
        "Operate CodeTether mux sessions directly. Use watch instead of wait_agent. Steer and interact return server-owned delivery acceptance; call watch or status for subsequent execution state."
    }
    fn parameters(&self) -> Value {
        schema::parameters()
    }
    async fn execute(&self, mut input: Value) -> Result<ToolResult> {
        let mut args = serde_json::from_value(input.clone())?;
        workspace::bind(&mut args, &mut input)?;
        if let Some(blocked) = unsafe_process_policy::blocked(&args, &input).await {
            return Ok(blocked);
        }
        crate::tool::network_access::guard!("mux_control", &input);
        execute::run(args).await
    }
}

#[cfg(test)]
#[path = "mux_control/tests.rs"]
mod tests;