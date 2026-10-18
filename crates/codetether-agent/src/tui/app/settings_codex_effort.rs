//! OpenAI Codex reasoning-effort control for the Settings panel.

use crate::provider::openai_codex::runtime_config;
use crate::session::Session;
use crate::tui::app::state::App;

/// Current Codex effort label displayed in Settings.
pub fn codex_thinking_effort_label() -> String {
    codetether_settings_policy::reasoning_label(runtime_config::thinking_level())
}

/// Cycle through the active Codex model's supported efforts.
pub async fn cycle_codex_thinking_effort(app: &mut App, session: &mut Session) {
    super::host::cycle(
        app,
        session,
        codetether_settings_policy::cycles::Cycle::CodexEffort,
    )
    .await;
}
