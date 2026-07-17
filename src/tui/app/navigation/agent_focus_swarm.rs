use crate::tui::app::state::App;

pub(super) fn sync(app: &mut App, name: Option<&str>) {
    let Some(index) = name.and_then(|name| {
        app.state
            .swarm
            .subtasks
            .iter()
            .position(|task| task.agent_name.as_deref() == Some(name))
    }) else {
        return;
    };
    app.state.swarm.selected_index = index;
    app.state.swarm.list_state.select(Some(index));
}
