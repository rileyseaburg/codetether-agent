//! Rows for workers owned by the active or most recent swarm.

use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};

use codetether_swarm_view::{SubTaskInfo, swarm_view_fmt::status_glyph};

/// Render one swarm subtask in the unified agent dashboard.
/// Format a child or worker dashboard row.
/// # Arguments
/// * `task` - Shared child snapshot.
/// * `selected` - Whether to show the selection marker.
/// # Returns
/// An owned display row.
/// # Examples
/// ```no_run
/// fn row(task: &codetether_swarm_view::SubTaskInfo) {
///     let _row = codetether_agent_dashboard::swarm_row::line(task, false);
/// }
/// ```
pub fn line(task: &SubTaskInfo, selected: bool) -> Line<'static> {
    let (glyph, color) = status_glyph(task.status);
    let worker = task.agent_name.as_deref().unwrap_or("unassigned");
    let mut spans = vec![
        Span::raw(if selected { "▶ " } else { "  " }),
        Span::styled(format!("{glyph} "), Style::default().fg(color)),
        format!("{worker} ").cyan().bold(),
        format!("[swarm:S{}] ", task.stage).magenta(),
        task.name.clone().dim(),
    ];
    if let Some(tool) = &task.current_tool {
        spans.push(format!(" · {tool}").yellow());
    }
    if task.steps > 0 {
        spans.push(format!(" · {}/{} steps", task.steps, task.max_steps).dim());
    }
    Line::from(spans)
}
