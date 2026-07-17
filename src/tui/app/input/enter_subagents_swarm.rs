use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

pub(super) fn contains(app: &App, name: &str) -> bool {
    index(app, name).is_some()
}

pub(super) fn open(app: &mut App, name: &str) -> bool {
    let Some(index) = index(app, name) else {
        return false;
    };
    app.state.swarm.selected_index = index;
    app.state.swarm.list_state.select(Some(index));
    app.state.set_view_mode(ViewMode::Swarm);
    app.state.swarm.enter_detail();
    app.state.status = format!("Swarm agent @{name} detail");
    true
}

fn index(app: &App, name: &str) -> Option<usize> {
    app.state
        .swarm
        .subtasks
        .iter()
        .position(|task| task.agent_name.as_deref() == Some(name))
}
