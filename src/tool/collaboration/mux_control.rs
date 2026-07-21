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
    async fn execute(&self, input: Value) -> Result<ToolResult> {
        execute::run(serde_json::from_value(input)?).await
    }
}

#[cfg(test)]
#[path = "mux_control/tests.rs"]
mod tests;
