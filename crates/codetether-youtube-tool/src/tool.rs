//! YouTube tool metadata and action entry point.
use super::YouTubeTool;
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl Tool for YouTubeTool {
    fn id(&self) -> &str {
        "youtube"
    }

    fn name(&self) -> &str {
        "YouTube"
    }

    fn description(&self) -> &str {
        "Publish content to YouTube. Actions: publish_episode (convert podcast episode to video and upload), \
         upload_video (upload an existing video file), status (check YouTube channel connectivity). \
         Uses the Voice API's YouTube integration with OAuth2 authentication."
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        self.execute_action(params).await
    }
}
