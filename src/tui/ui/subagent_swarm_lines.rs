//! Dashboard section for swarm-owned workers.

use ratatui::style::Stylize;
use ratatui::text::Line;

use crate::tui::swarm_view::SwarmViewState;

/// Append current or most recent swarm subtasks.
pub fn append(rows: &mut Vec<Line<'static>>, swarm: &SwarmViewState) {
    if swarm.subtasks.is_empty() {
        return;
    }
    let state = if swarm.active { "live" } else { "most recent" };
    rows.push(Line::from(
        format!("Swarm workers · {state} · {}", swarm.task)
            .cyan()
            .bold(),
    ));
    rows.extend(swarm.subtasks.iter().map(|task| {
        let selected = task.agent_name.as_deref() == swarm_focus_name(swarm);
        super::subagent_swarm_row::line(task, selected)
    }));
    rows.push(Line::from("Open /swarm for details and controls.".dim()));
}

fn swarm_focus_name(swarm: &SwarmViewState) -> Option<&str> {
    swarm
        .subtasks
        .get(swarm.selected_index)
        .and_then(|task| task.agent_name.as_deref())
}
