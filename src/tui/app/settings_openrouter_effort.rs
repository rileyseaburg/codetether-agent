//! OpenRouter reasoning-effort control for the Settings panel.

use crate::provider::openrouter::{reasoning_levels, runtime_config};
use crate::session::Session;
use crate::tui::app::state::App;

use super::persist;

/// Current OpenRouter effort label displayed in Settings.
pub fn openrouter_thinking_effort_label() -> String {
    runtime_config::thinking_level().unwrap_or_else(|| "default".to_string())
}

/// Cycle `default -> none -> minimal -> low -> medium -> high -> xhigh -> max`.
///
/// `none` stays in the cycle because most OpenRouter models honour it; the
/// request builder drops it for models that mandate reasoning (Grok 4.5/4.6)
/// rather than letting the turn fail with HTTP 400.
pub async fn cycle_openrouter_thinking_effort(app: &mut App, session: &mut Session) {
    let levels = reasoning_levels::LEVELS;
    let next = runtime_config::thinking_level()
        .as_deref()
        .and_then(|current| levels.iter().position(|level| *level == current))
        .map_or(Some(levels[0]), |index| levels.get(index + 1).copied());
    runtime_config::set_thinking_level(next.map(String::from));
    persist(
        app,
        session,
        format!(
            "OpenRouter thinking effort: {}",
            openrouter_thinking_effort_label()
        ),
    )
    .await;
}

#[cfg(test)]
#[path = "settings_openrouter_effort_tests.rs"]
mod tests;
