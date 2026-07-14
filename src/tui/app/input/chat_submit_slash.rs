//! Slash-command and `!shell` prefix dispatch for chat submit.

use std::path::Path;
use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tui::app::commands::handle_slash_command;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;

#[path = "mux_command/mod.rs"]
mod mux_command;
#[path = "shell/mod.rs"]
mod shell;
#[path = "slash_no_session.rs"]
mod slash_no_session;
#[cfg(test)]
#[path = "slash_no_session_tests.rs"]
mod slash_no_session_tests;

pub(super) async fn run(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    prompt: &str,
) -> bool {
    if shell::run(app, cwd, prompt).await {
        return true;
    }
    if !prompt.starts_with('/') {
        return false;
    }
    if super::approval_command::run(app, prompt) {
        return true;
    }
    if mux_command::run(app, cwd, prompt).await {
        return true;
    }
    let Some(session) = slot.borrow_mut() else {
        // The session is checked out by an in-flight prompt. UI-only
        // commands (/settings, /model, /file) still work because they
        // touch app.state, not the session.
        if slash_no_session::run(app, cwd, registry, prompt) {
            app.state.clear_input();
        } else {
            app.state.status = "Session is busy; slash command was not run".to_string();
        }
        return true;
    };
    if super::codex_parity_command::run(app, cwd, session, registry.as_ref(), prompt).await {
        return true;
    }
    handle_slash_command(app, cwd, session, registry.as_ref(), prompt).await;
    app.state.clear_input();
    true
}
