//! Action dispatch for the VoiceStream tool.

mod audio_player;
mod play;
mod speak_stream;
mod tts_request;
mod tts_result;

use super::{Params, ToolResult};
use anyhow::Result;
use serde_json::json;

/// Route to the correct action handler based on `params.action`.
pub(crate) async fn dispatch(client: &reqwest::Client, params: &Params) -> Result<ToolResult> {
    match params.action.as_str() {
        "speak_stream" => speak_stream::run(client, params).await,
        "play" => play::run(client, params).await,
        other => Ok(ToolResult::structured_error(
            "INVALID_ACTION",
            "voice_stream",
            &format!("Unknown action '{other}'. Use: speak_stream, play"),
            None,
            Some(json!({"action": "speak_stream", "text": "Hello"})),
        )),
    }
}
