//! Tool trait implementation for `context_pin`.

use super::super::{Tool, ToolResult};
use super::super::context_helpers::load_latest_session;
use super::logic::apply_pin;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

/// Pin or unpin a turn by message index.
pub struct ContextPinTool;

#[async_trait]
impl Tool for ContextPinTool {
    fn id(&self) -> &str { "context_pin" }
    fn name(&self) -> &str { "ContextPin" }

    fn description(&self) -> &str {
        "Pin or unpin a conversation turn so it is never dropped during \
         context compression. Pinned turns are treated as hard constraints. \
         Pass `action: \"pin\"` or `action: \"unpin\"` along with the \
         0-based `turn_index`. Pinned turns survive across resets."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": { "type": "string", "enum": ["pin", "unpin"],
                    "description": "Whether to pin or unpin the turn." },
                "turn_index": { "type": "integer", "minimum": 0,
                    "description": "0-based index into the session transcript." }
            },
            "required": ["action", "turn_index"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let action = args["action"].as_str().unwrap_or("").to_lowercase();
        let idx = match args["turn_index"].as_u64() {
            Some(i) => i as usize,
            None => return Ok(ToolResult::error("`turn_index` must be a non-negative integer.")),
        };
        let mut session = match load_latest_session().await {
            Ok(Some(s)) => s,
            Ok(None) => return Ok(ToolResult::error("No active session.")),
            Err(e) => return Ok(ToolResult::error(&format!("Load failed: {e}"))),
        };
        match apply_pin(&mut session, idx, &action).await {
            Ok(msg) => Ok(ToolResult::success(msg)),
            Err(e) => Ok(ToolResult::error(&e.to_string())),
        }
    }
}
