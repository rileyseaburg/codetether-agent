//! Inspect recent agent-bus activity on demand.
//!
//! Reads from the process-global [`AgentBus`](codetether_bus_core::AgentBus) ring buffer
//! so callers can see recent envelopes (swarm events, task updates, shared
//! results) without pre-attaching a live subscriber.

mod execute;
mod schema;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

use codetether_tool_core::{Tool, ToolResult};

/// Tool exposing recent bus envelopes.
///
/// # Examples
/// ```rust
/// let _tool = codetether_bus_tool::BusInspectTool;
/// ```
pub struct BusInspectTool;

#[async_trait]
impl Tool for BusInspectTool {
    fn id(&self) -> &str {
        "bus_inspect"
    }

    fn name(&self) -> &str {
        "bus_inspect"
    }

    fn description(&self) -> &str {
        "Inspect recent agent-bus activity (swarm/task/shared-result events) \
         from the in-memory ring buffer. Optional topic_prefix filter and limit."
    }

    fn parameters(&self) -> Value {
        schema::parameters()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        execute::execute(args).await
    }
}
