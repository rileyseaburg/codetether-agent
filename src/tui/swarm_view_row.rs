//! Single-row span construction for a swarm subtask.

use super::SubTaskInfo;
use super::swarm_view_fmt::{elapsed_label, status_glyph};
use crate::swarm::SubTaskStatus;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Build the styled line for one subtask. `show_divider` prefixes a stage
/// marker when this row begins a new stage.
pub(crate) fn task_line(task: &SubTaskInfo, show_divider: bool) -> Line<'static> {
    let (icon, color) = status_glyph(task.status);
    let stage_tag = if show_divider {
        format!("── S{} ─ ", task.stage)
    } else {
        "       ".to_string()
    };
    let mut spans = vec![
        Span::styled(stage_tag, Style::default().fg(Color::Gray)),
        Span::styled(format!("{icon} "), Style::default().fg(color)),
        Span::styled(task.name.clone(), Style::default().fg(Color::White)),
    ];
    if task.status == SubTaskStatus::Running
        && let Some(agent) = &task.agent_name
    {
        spans.push(Span::styled(
            format!(" → {agent}"),
            Style::default().fg(Color::Cyan),
        ));
    }
    if let Some(tool) = &task.current_tool {
        spans.push(Span::styled(
            format!(" [{tool}]"),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::DIM),
        ));
    }
    if let Some(elapsed) = elapsed_label(task) {
        spans.push(Span::styled(
            format!("  {elapsed}"),
            Style::default().fg(Color::Gray),
        ));
    }
    if task.steps > 0 {
        spans.push(Span::styled(
            format!("  ({}/{})", task.steps, task.max_steps),
            Style::default().fg(Color::Gray),
        ));
    }
    Line::from(spans)
}
