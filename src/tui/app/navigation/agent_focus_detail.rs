//! Child-only focus cycling while a transcript detail pane is open.

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

pub(super) fn cycle(app: &mut App) {
    super::child::next(app);
    open_swarm_detail(app);
    app.state.subagent_detail_scroll = 0;
}

pub(super) fn cycle_back(app: &mut App) {
    super::child::previous(app);
    open_swarm_detail(app);
    app.state.subagent_detail_scroll = 0;
}

fn open_swarm_detail(app: &mut App) {
    let selected = app
        .state
        .swarm
        .selected_subtask()
        .and_then(|task| task.agent_name.as_deref());
    if selected == app.state.active_spawned_agent.as_deref() {
        app.state.subagent_detail_mode = false;
        app.state.set_view_mode(ViewMode::Swarm);
        app.state.swarm.enter_detail();
    }
}

#[cfg(test)]
#[path = "agent_focus_detail_tests.rs"]
mod tests;
