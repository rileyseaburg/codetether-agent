use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

pub(super) fn focus(app: &mut App, query: &str) -> bool {
    let matches = app
        .state
        .swarm
        .subtasks
        .iter()
        .enumerate()
        .filter(|(_, task)| {
            task.agent_name
                .as_deref()
                .is_some_and(|name| name == query || name.starts_with(query))
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let [index] = matches.as_slice() else {
        if matches.len() > 1 {
            app.state.status = format!("Ambiguous swarm agent prefix: {query}");
            return true;
        }
        return false;
    };
    app.state.swarm.selected_index = *index;
    app.state.swarm.list_state.select(Some(*index));
    app.state.swarm.enter_detail();
    app.state.set_view_mode(ViewMode::Swarm);
    app.state.active_spawned_agent = app.state.swarm.subtasks[*index].agent_name.clone();
    app.state.status = format!("Focused swarm agent: @{query}");
    true
}

#[cfg(test)]
#[path = "managed_agent_swarm_tests.rs"]
mod tests;
