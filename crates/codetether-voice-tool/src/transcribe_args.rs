//! Validate arguments and invoke the transcribe action.
use super::{
    VoiceTool,
    params::{Params, TranscribeParams},
};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::json;
impl VoiceTool {
    pub(super) async fn transcribe_args(&self, p: Params) -> Result<ToolResult> {
        let file_path = match p.file_path {
            Some(f) if !f.trim().is_empty() => f,
            _ => {
                return Ok(ToolResult::structured_error(
                    "MISSING_PARAM",
                    "voice",
                    "The 'file_path' parameter is required for the 'transcribe' action",
                    Some(vec!["file_path"]),
                    Some(json!({"action": "transcribe", "file_path": "/path/to/audio.wav"})),
                ));
            }
        };
        self.transcribe(&TranscribeParams { file_path }).await
    }
}
