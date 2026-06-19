//! Inspect recent agent-bus activity on demand.
//!
//! Reads from the process-global [`AgentBus`](crate::bus::AgentBus) ring buffer
//! so callers can see recent envelopes (swarm events, task updates, shared
//! results) without pre-attaching a live subscriber.

mod schema;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

use super::{Tool, ToolResult};

/// Tool exposing recent bus envelopes.
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
        let limit = args
            .get("limit")
            .and_then(Value::as_u64)
            .unwrap_or(20)
            .min(200) as usize;
        let prefix = args.get("topic_prefix").and_then(Value::as_str);

        let Some(bus) = crate::bus::global() else {
            return Ok(ToolResult::error(
                "No global agent bus is installed in this process.",
            ));
        };

        let envelopes = bus.recorder.recent(limit, prefix);
        let count = envelopes.len();
        let body = json!({ "count": count, "envelopes": envelopes });
        Ok(ToolResult::success(serde_json::to_string_pretty(&body)?)
            .with_metadata("count", json!(count)))
    }
}
