//! Bedrock thinking-effort cycling for the Settings panel.
//!
//! Controls the `CODETETHER_BEDROCK_THINKING_EFFORT` env var that feeds
//! `output_config.effort` on encrypted-reasoning models (Claude Fable 5,
//! Opus 4.7). Higher effort → more visible reasoning before tool calls.

use crate::session::Session;
use crate::tui::app::state::App;

use super::persist;

/// Current effort label as shown in the Settings panel.
pub fn bedrock_thinking_effort_label() -> &'static str {
    match std::env::var("CODETETHER_BEDROCK_THINKING_EFFORT")
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "low" => "low",
        "high" => "high",
        _ => "medium",
    }
}

/// Cycle: medium → low → high → medium (default is medium).
pub async fn cycle_bedrock_thinking_effort(app: &mut App, session: &mut Session) {
    let next = match bedrock_thinking_effort_label() {
        "medium" => "low",
        "low" => "high",
        _ => "medium",
    };
    unsafe { std::env::set_var("CODETETHER_BEDROCK_THINKING_EFFORT", next) };
    persist(
        app,
        session,
        format!("Bedrock thinking effort: {next}"),
    )
    .await;
}
