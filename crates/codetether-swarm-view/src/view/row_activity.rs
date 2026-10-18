//! Agent identity, active tool, elapsed time and step annotations.
use super::swarm_view_fmt::elapsed_label;
use crate::SubTaskInfo;
use codetether_swarm_core::SubTaskStatus;
use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};
pub(super) fn append(task: &SubTaskInfo, spans: &mut Vec<Span<'static>>) {
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
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::DIM),
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
}
