//! Swarm detail header presentation.
use crate::SubTaskInfo;
use codetether_swarm_core::SubTaskStatus;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
pub(super) fn render(f: &mut Frame, task: &SubTaskInfo, area: Rect) {
    // --- Header: agent info ---
    let (status_icon, status_color) = match task.status {
        SubTaskStatus::Pending => ("○ Pending", Color::DarkGray),
        SubTaskStatus::Blocked => ("⊘ Blocked", Color::Yellow),
        SubTaskStatus::Running => ("● Running", Color::Cyan),
        SubTaskStatus::Completed => ("✓ Completed", Color::Green),
        SubTaskStatus::Failed => ("✗ Failed", Color::Red),
        SubTaskStatus::Cancelled => ("⊗ Cancelled", Color::DarkGray),
        SubTaskStatus::TimedOut => ("⏱ Timed Out", Color::Red),
    };

    let agent_label = task.agent_name.as_deref().unwrap_or("(unassigned)");
    let header_lines = vec![
        super::task_identity::row(task),
        super::task_status::row(task, agent_label, status_icon, status_color),
        super::task_dependencies::row(task),
    ];

    let title = format!(" Agent Detail: {} ", task.id);
    let header = Paragraph::new(header_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(status_color)),
    );
    f.render_widget(header, area);
}
