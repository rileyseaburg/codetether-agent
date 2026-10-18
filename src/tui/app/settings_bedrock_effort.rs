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

/// Current effort label as shown in the Settings panel.
pub fn bedrock_thinking_effort_label() -> &'static str {
    codetether_settings_policy::bedrock_thinking_effort_label(
        runtime_config::thinking_effort()
            .unwrap_or_default()
            .as_str(),
    )
}

/// Cycle: medium → low → high → medium (default is medium).
pub async fn cycle_bedrock_thinking_effort(app: &mut App, session: &mut Session) {
    super::host::cycle(
        app,
        session,
        codetether_settings_policy::cycles::Cycle::BedrockEffort,
    )
    .await;
}
