//! Dispatch for the Settings panel toggle/Enter action.
//!
//! Maps `selected_settings_index` to the corresponding setting action.
//! Keeping this in its own module respects the 50-line file limit and
//! separates dispatch from individual setting implementations.

use crate::session::Session;
use crate::tui::app::commands::set_auto_apply_edits;
use crate::tui::app::state::App;

use super::access_mode;
use super::bedrock::cycle_bedrock_service_tier;
use super::bedrock_effort::cycle_bedrock_thinking_effort;
use super::network;
use super::{set_slash_autocomplete, set_use_worktree};

/// Execute the action for the currently selected settings row.
pub async fn toggle_selected_setting(app: &mut App, session: &mut Session) {
    match app.state.selected_settings_index {
        0 => set_auto_apply_edits(app, session, !app.state.auto_apply_edits).await,
        1 => network::set_network_access(app, session, !app.state.allow_network).await,
        2 => set_slash_autocomplete(app, session, !app.state.slash_autocomplete).await,
        3 => set_use_worktree(app, session, !app.state.use_worktree).await,
        4 => access_mode::cycle_access_mode(app, session).await,
        5 => cycle_bedrock_service_tier(app, session).await,
        6 => cycle_bedrock_thinking_effort(app, session).await,
        _ => {}
    }
}
