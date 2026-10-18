//! Single-row span construction for a swarm subtask.

use super::swarm_view_fmt::status_glyph;
use crate::SubTaskInfo;
use ratatui::{
    style::{Color, Style},
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
    super::row_activity::append(task, &mut spans);
    Line::from(spans)
}
