//! Session-free slash commands usable while a prompt is processing.
//!
//! When a turn is in flight the canonical [`Session`](crate::session::Session)
//! is checked out of the [`SessionSlot`](super::super::session_runtime::SessionSlot),
//! so session-touching commands cannot run. The UI-only commands here only
//! mutate `app.state`, so they stay available during processing — letting the
//! user open `/settings`, switch `/model`, or attach a `/file` mid-response.

use std::path::Path;
use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

use self::helpers::{attach_file_command, open_model_view};

#[path = "slash_no_session_helpers.rs"]
mod helpers;

/// Run a UI-only slash command when no session is available.
///
/// Returns `true` when `prompt` matched a UI-only command (handled here),
/// or `false` when the caller should report the session as busy.
pub(super) fn run(
    app: &mut App,
    cwd: &Path,
    registry: &Option<Arc<ProviderRegistry>>,
    prompt: &str,
) -> bool {
    let normalized = crate::tui::app::text::normalize_slash_command(prompt);
    if let Some(rest) = crate::tui::app::text::command_with_optional_args(&normalized, "/file") {
        attach_file_command(
            app,
            cwd,
            rest.trim().trim_matches(|c| c == '"' || c == '\''),
        );
        return true;
    }
    match normalized.as_str() {
        "/settings" => app.state.set_view_mode(ViewMode::Settings),
        "/model" => open_model_view(app, registry),
        "/help" => {
            app.state.show_help = true;
            app.state.help_scroll.offset = 0;
            app.state.status = "Help".to_string();
        }
        _ => return false,
    }
    true
}
