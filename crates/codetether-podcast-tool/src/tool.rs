//! Tool metadata and execution entry point.
use super::PodcastTool;
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl Tool for PodcastTool {
    fn id(&self) -> &str {
        "podcast"
    }

    fn name(&self) -> &str {
        "Podcast"
    }

    fn description(&self) -> &str {
        "Create and manage AI-generated podcasts with cloned voice narration. \
         Actions: create_podcast (new podcast series), create_episode (generate TTS episode from script), \
         list_podcasts (show all podcasts), get_podcast (details + episodes), \
         delete_podcast, delete_episode. Episodes are auto-converted to MP3 with RSS feed generation."
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, params: Value) -> Result<ToolResult> {
        self.execute_action(params).await
    }
}
