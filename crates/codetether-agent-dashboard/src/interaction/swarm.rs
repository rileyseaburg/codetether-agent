//! Swarm name lookup and synchronized detail selection.
use super::Host;

pub(super) fn contains(app: &impl Host, name: &str) -> bool {
    index(app, name).is_some()
}

pub(super) fn open(app: &mut impl Host, name: &str) -> bool {
    let Some(index) = index(app, name) else {
        return false;
    };
    app.swarm_mut().selected_index = index;
    app.swarm_mut().list_state.select(Some(index));
    app.show_swarm();
    app.swarm_mut().enter_detail();
    app.set_status(format!("Swarm agent @{name} detail"));
    true
}

fn index(app: &impl Host, name: &str) -> Option<usize> {
    app.swarm()
        .subtasks
        .iter()
        .position(|task| task.agent_name.as_deref() == Some(name))
}
