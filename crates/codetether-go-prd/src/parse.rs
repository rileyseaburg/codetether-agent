//! Interpret recovered JSON as a PRD and return repair feedback.

use codetether_ralph_core::Prd;
pub(crate) fn response(last_text: &str) -> Result<Prd, String> {
    match crate::extract::extract_json(last_text) {
        Some(json_str) => serde_json::from_str::<Prd>(&json_str)
            .map_err(|e| format!("JSON parses but doesn't match PRD schema: {e}")),
        None => Err("Response contains no valid JSON object".to_string()),
    }
}
