//! In-memory generic mutator used by approval retry tests.

use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
pub(super) struct CountingTool(AtomicUsize);

impl CountingTool {
    pub(super) fn count(&self) -> usize {
        self.0.load(Ordering::SeqCst)
    }
}

#[async_trait]
impl Tool for CountingTool {
    fn id(&self) -> &str {
        "generic_mutator"
    }

    fn name(&self) -> &str {
        "Generic Mutator"
    }

    fn description(&self) -> &str {
        "Approval retry test tool"
    }

    fn parameters(&self) -> Value {
        serde_json::json!({"type": "object"})
    }

    async fn execute(&self, _args: Value) -> Result<ToolResult> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(ToolResult::success("executed"))
    }
}
