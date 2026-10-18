//! Dashboard section for swarm-owned workers.

use ratatui::style::Stylize;
use ratatui::text::Line;

use codetether_swarm_view::SwarmViewState;

/// Append current or most recent swarm subtasks.
/// Append the current or most recent swarm section.
/// # Arguments
/// * `rows` - Destination dashboard rows.
/// * `swarm` - Shared swarm state.
/// # Returns
/// Appends rows when subtasks exist.
/// # Examples
/// ```
/// let mut rows = Vec::new();
/// codetether_agent_dashboard::swarm::append(&mut rows, &codetether_swarm_view::SwarmViewState::default());
/// assert!(rows.is_empty());
/// ```
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
        crate::swarm_row::line(task, selected)
    }));
    rows.push(Line::from("Open /swarm for details and controls.".dim()));
}

fn swarm_focus_name(swarm: &SwarmViewState) -> Option<&str> {
    swarm
        .subtasks
        .get(swarm.selected_index)
        .and_then(|task| task.agent_name.as_deref())
}
