//! Action dispatch for the VoiceInput tool.

pub mod record_then_transcribe;

pub mod transcribe_api;

use super::params::Params;
use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::json;

/// Route to the correct action handler based on `params.action`.
pub(crate) async fn dispatch(client: &reqwest::Client, params: &Params) -> Result<ToolResult> {
    match params.action.as_str() {
        "record_then_transcribe" => {
            record_then_transcribe::run(client, params.max_duration_secs).await
        }
        other => Ok(ToolResult::structured_error(
            "INVALID_ACTION",
            "voice_input",
            &format!("Unknown action '{other}'. Use: record_then_transcribe"),
            None,
            Some(json!({"action": "record_then_transcribe"})),
        )),
    }
}
