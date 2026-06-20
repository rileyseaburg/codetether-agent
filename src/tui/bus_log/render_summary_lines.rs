//! Line construction for the protocol summary panel.

use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span},
};

use super::render_summary_data::SummaryData;

pub(super) fn lines(data: &SummaryData) -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            "A2A worker: ".dim(),
            colored(data.worker_label, data.worker_color),
            sep(),
            Span::raw(data.worker_name.clone()).cyan(),
            sep(),
            Span::raw(data.worker_id.clone()).dim(),
        ]),
        Line::from(vec![
            "A2A peer: ".dim(),
            colored(data.peer_label, data.peer_color),
            sep(),
            Span::raw(format!("{} visible A2A msg(s)", data.a2a_count)),
        ]),
        Line::from(vec![
            "Heartbeat: ".dim(),
            colored(data.processing_label, data.processing_color),
            sep(),
            Span::raw(format!("{} queued task(s)", data.queued_tasks)),
        ]),
        Line::from(vec![
            "Agents: ".dim(),
            Span::raw(data.registered_agents.clone()),
        ]),
        Line::from(vec![
            "Workspace: ".dim(),
            Span::raw(data.cwd_display.clone()),
        ]),
        Line::from(vec![
            "Recent task: ".dim(),
            Span::raw(data.recent_task.clone()),
        ]),
    ]
}

fn colored(text: &'static str, color: ratatui::style::Color) -> Span<'static> {
    Span::styled(text, Style::default().fg(color).bold())
}

fn sep() -> Span<'static> {
    "  •  ".dim()
}
