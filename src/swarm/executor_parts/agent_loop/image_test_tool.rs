//! Deterministic tool returning caller-selected text and image metadata.

use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

pub(super) struct ImageTool(pub ToolResult);

#[async_trait]
impl Tool for ImageTool {
    fn id(&self) -> &str {
        "image_fixture"
    }
    fn name(&self) -> &str {
        self.id()
    }
    fn description(&self) -> &str {
        "Offline image result fixture"
    }
    fn parameters(&self) -> Value {
        json!({"type": "object"})
    }
    async fn execute(&self, _: Value) -> Result<ToolResult> {
        Ok(self.0.clone())
    }
}
