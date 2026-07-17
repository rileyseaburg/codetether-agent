//! Child-only focus cycling for the subagent dashboard.

use crate::tui::app::state::App;

/// Select the next child, wrapping from the last child to the first.
pub(super) fn next(app: &mut App) {
    cycle(app, Direction::Forward);
}

/// Select the previous child, wrapping from the first child to the last.
pub(super) fn previous(app: &mut App) {
    cycle(app, Direction::Backward);
}

#[derive(Clone, Copy)]
enum Direction {
    Forward,
    Backward,
}

fn cycle(app: &mut App, direction: Direction) {
    let names = super::agent_names(app);
    if names.is_empty() {
        app.state.active_spawned_agent = None;
        app.state.status = "No spawned agents. Use /spawn <name> to create one.".to_string();
        return;
    }

    let current = app
        .state
        .active_spawned_agent
        .as_ref()
        .and_then(|name| names.iter().position(|candidate| candidate == name));
    let index = match (direction, current) {
        (Direction::Forward, Some(index)) => (index + 1) % names.len(),
        (Direction::Backward, Some(0)) => names.len() - 1,
        (Direction::Backward, Some(index)) => index - 1,
        (Direction::Forward, None) => 0,
        (Direction::Backward, None) => names.len() - 1,
    };
    super::set_focus(app, Some(names[index].clone()));
}
