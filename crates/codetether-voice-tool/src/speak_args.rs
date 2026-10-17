//! Validate arguments and invoke the speak action.
use super::{
    VoiceTool,
    params::{Params, SpeakParams},
};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::json;
impl VoiceTool {
    pub(super) async fn speak_args(&self, p: Params) -> Result<ToolResult> {
        let text = match p.text {
            Some(t) if !t.trim().is_empty() => t,
            _ => {
                return Ok(ToolResult::structured_error(
                    "MISSING_PARAM",
                    "voice",
                    "The 'text' parameter is required for the 'speak' action",
                    Some(vec!["text"]),
                    Some(json!({"action": "speak", "text": "Hello world"})),
                ));
            }
        };
        self.speak(&SpeakParams {
            text,
            voice_id: p.voice_id,
            language: p.language,
        })
        .await
    }
}
