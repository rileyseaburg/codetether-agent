//! OpenAI Codex reasoning-effort control for the Settings panel.

use crate::provider::openai_codex::{reasoning_catalog, runtime_config};
use crate::session::Session;
use crate::tui::app::state::App;

use super::persist;

/// Current Codex effort label displayed in Settings.
pub fn codex_thinking_effort_label() -> String {
    runtime_config::thinking_level().unwrap_or_else(|| "default".to_string())
}

/// Cycle through the active Codex model's supported efforts.
pub async fn cycle_codex_thinking_effort(app: &mut App, session: &mut Session) {
    let model = session.metadata.model.as_deref().unwrap_or_default();
    let levels = reasoning_catalog::supported_levels(model);
    if levels.is_empty() {
        app.state.status = "Select an OpenAI Codex model before changing its effort".to_string();
        return;
    }
    let current = runtime_config::thinking_level();
    let next = current
        .as_deref()
        .and_then(|value| levels.iter().position(|level| *level == value))
        .and_then(|index| levels.get(index + 1))
        .copied();
    let next = current.is_none().then_some(levels[0]).or(next);
    runtime_config::set_thinking_level(next.map(String::from));
    persist(
        app,
        session,
        format!("Codex thinking effort: {}", codex_thinking_effort_label()),
    )
    .await;
}
