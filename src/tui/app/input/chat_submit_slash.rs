//! Slash-command dispatch for chat submit.

use std::path::Path;
use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tui::app::commands::handle_slash_command;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;

pub(super) async fn run(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    prompt: &str,
) -> bool {
    if !prompt.starts_with('/') {
        return false;
    }
    let Some(session) = slot.borrow_mut() else {
        app.state.status = "Session is busy; slash command was not run".to_string();
        return true;
    };
    handle_slash_command(app, cwd, session, registry.as_ref(), prompt).await;
    app.state.clear_input();
    true
}
