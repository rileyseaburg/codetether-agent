//! Tab-key cycling of the active spawned-agent focus.

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

#[path = "agent_focus_back.rs"]
mod back;
#[path = "agent_focus_child.rs"]
mod child;
#[path = "agent_focus_detail.rs"]
mod detail;
#[path = "agent_focus_set.rs"]
mod focus;
#[path = "agent_focus_main.rs"]
mod main;
#[path = "agent_focus_names.rs"]
mod names;
#[path = "agent_focus_swarm.rs"]
mod swarm;
pub use back::cycle_agent_focus_back;
use focus::set_focus;
pub use main::cycle_agent_focus;
use names::agent_names;

/// Accept a slash suggestion or cycle the applicable focus ring.
pub fn handle_tab(app: &mut App) {
    if app.state.apply_selected_slash_suggestion() {
        app.state.status = "Command autocompleted".to_string();
    } else if app.state.view_mode == ViewMode::Subagents {
        if app.state.subagent_detail_mode {
            detail::cycle(app);
        } else {
            child::next(app);
        }
        app.state.subagent_detail_scroll = 0;
    } else {
        cycle_agent_focus(app);
    }
}

pub(super) fn cycle_child_focus(app: &mut App) {
    child::next(app);
}

pub(super) fn cycle_child_focus_back(app: &mut App) {
    child::previous(app);
}
