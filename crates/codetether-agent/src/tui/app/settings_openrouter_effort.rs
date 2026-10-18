//! OpenRouter reasoning-effort control for the Settings panel.

use crate::provider::openrouter::runtime_config;
use crate::session::Session;
use crate::tui::app::state::App;

/// Current OpenRouter effort label displayed in Settings.
pub fn openrouter_thinking_effort_label() -> String {
    codetether_settings_policy::reasoning_label(runtime_config::thinking_level())
}

/// Cycle `default -> none -> minimal -> low -> medium -> high -> xhigh -> max`.
///
/// `none` stays in the cycle because most OpenRouter models honour it; the
/// request builder drops it for models that mandate reasoning (Grok 4.5/4.6)
/// rather than letting the turn fail with HTTP 400.
pub async fn cycle_openrouter_thinking_effort(app: &mut App, session: &mut Session) {
    super::host::cycle(
        app,
        session,
        codetether_settings_policy::cycles::Cycle::OpenRouterEffort,
    )
    .await;
}

#[cfg(test)]
#[path = "settings_openrouter_effort_tests.rs"]
mod tests;
