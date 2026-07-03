//! Bedrock thinking-effort cycling for the Settings panel.
//!
//! Controls the thinking effort that feeds `output_config.effort` on
//! encrypted-reasoning models (Claude Fable 5, Opus 4.7). Higher effort →
//! more visible reasoning before tool calls. Stored in thread-safe process
//! state (see [`crate::provider::bedrock::runtime_config`]) rather than a
//! runtime-mutated env var.

use crate::provider::bedrock::runtime_config;
use crate::session::Session;
use crate::tui::app::state::App;

use super::persist;

/// Current effort label as shown in the Settings panel.
pub fn bedrock_thinking_effort_label() -> &'static str {
    match runtime_config::thinking_effort().unwrap_or_default().as_str() {
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
    runtime_config::set_thinking_effort(Some(next.to_string()));
    persist(app, session, format!("Bedrock thinking effort: {next}")).await;
}
