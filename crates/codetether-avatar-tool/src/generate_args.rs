//! Validate generate arguments and dispatch its request.
use super::{AvatarTool, params::*};
use anyhow::Result;
use codetether_tool_core::ToolResult;
impl AvatarTool {
    pub(super) async fn generate_args(&self, p: Params) -> Result<ToolResult> {
        self.generate(&GenerateParams {
            text: p.text,
            audio_url: p.audio_url,
            audio_file: p.audio_file,
            model_video: p.model_video,
            voice_id: p.voice_id,
        })
        .await
    }
}
