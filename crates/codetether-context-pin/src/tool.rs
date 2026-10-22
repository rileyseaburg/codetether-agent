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
        "Retain complete source text across compression and restarts. Use pin or unpin with an absolute turn_index. \
         Use supersede with turn_index and a later user-message replacement_index only when the user replaces all requirements in that pinned source. \
         For a partial correction, use correct with exact unique source_text, source_hash from the pinned header, and replacement_index. This replaces only that span, preserving all other original requirements. \
         Superseded text remains in canonical history, but only the replacement is active. \
         Pinning assistant or tool evidence never turns it into user authorization."
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        super::execute::run::<S>(args).await
    }
}
