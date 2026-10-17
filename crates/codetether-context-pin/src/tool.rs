//! Pin tool metadata and typed execution.
use crate::{ContextPinTool, PinSource};
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl<S: PinSource> Tool for ContextPinTool<S> {
    fn id(&self) -> &str {
        "context_pin"
    }

    fn name(&self) -> &str {
        "ContextPin"
    }

    fn description(&self) -> &str {
        "Pin or unpin a conversation turn so it is never dropped during \
         context compression. Pinned turns are treated as hard constraints. \
         Pass `action: \"pin\"` or `action: \"unpin\"` along with the \
         0-based `turn_index`. Pinned turns survive across resets."
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        super::execute::run::<S>(args).await
    }
}
