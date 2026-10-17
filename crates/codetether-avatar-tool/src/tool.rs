//! Tool metadata and execution entry point.
use super::AvatarTool;
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl Tool for AvatarTool {
    fn id(&self) -> &str {
        "avatar"
    }

    fn name(&self) -> &str {
        "Avatar"
    }

    fn description(&self) -> &str {
        "AI digital human video generation using Duix Avatar. Create lip-synced avatar videos \
         from audio or text. Actions: status (check service), start/stop (manage GPU container), \
         generate (create avatar video from audio), generate_from_episode (podcast → avatar video → YouTube), \
         list_models (show available avatar models), upload_model (upload a silent video for cloning). \
         Supports the full pipeline: text → speech → avatar video → YouTube."
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        self.execute_action(params).await
    }
}
