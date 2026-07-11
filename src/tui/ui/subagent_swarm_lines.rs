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
    let mut tasks = swarm.subtasks.iter().collect::<Vec<_>>();
    tasks.sort_by_key(|task| (task.stage, &task.name));
    rows.extend(tasks.into_iter().map(super::subagent_swarm_row::line));
    rows.push(Line::from("Open /swarm for details and controls.".dim()));
}
