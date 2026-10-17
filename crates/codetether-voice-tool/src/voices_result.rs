//! Format voice-profile listings.
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
pub(super) fn format(body: Value) -> ToolResult {
    let voices = body["voices"].as_array();

    match voices {
        Some(voices) if !voices.is_empty() => {
            let mut output = String::from("Available voices:\n\n");
            for v in voices {
                let id = v["voice_id"].as_str().unwrap_or("?");
                let name = v["name"].as_str().unwrap_or("?");
                let dur = v["duration_seconds"].as_f64().unwrap_or(0.0);
                let created = v["created_at"].as_str().unwrap_or("?");
                output.push_str(&format!(
                    "- {name} (id: {id}, sample: {dur:.1}s, created: {created})\n"
                ));
            }
            ToolResult::success(output).with_metadata("count", json!(voices.len()))
        }
        _ => ToolResult::success("No voices found. Create one by uploading a voice sample."),
    }
}
