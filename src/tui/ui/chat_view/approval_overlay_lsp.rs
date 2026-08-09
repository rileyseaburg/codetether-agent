//! Real language-server diagnostics panel for proposed approval content.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::app::state::approval_queue::{ApprovalReport, ApprovalReportState};

pub(super) fn height(report: &ApprovalReport) -> u16 {
    match report.state {
        ApprovalReportState::NotRequested => 0,
        ApprovalReportState::Issues => (report.messages.len() as u16 + 2).min(5),
        ApprovalReportState::Checking
        | ApprovalReportState::Clean
        | ApprovalReportState::Unavailable => 3,
    }
}

pub(super) fn render(f: &mut Frame, area: Rect, report: &ApprovalReport) {
    if matches!(report.state, ApprovalReportState::NotRequested) {
        return;
    }
    let (title, color) = match report.state {
        ApprovalReportState::Checking => ("LSP · checking proposed content…", Color::Yellow),
        ApprovalReportState::Clean => ("LSP · no diagnostics", Color::Green),
        ApprovalReportState::Issues => ("LSP · proposed-content diagnostics", Color::Red),
        ApprovalReportState::Unavailable => ("LSP · unavailable", Color::DarkGray),
        ApprovalReportState::NotRequested => return,
    };
    let lines = if report.messages.is_empty() {
        vec![Line::raw(title)]
    } else {
        report
            .messages
            .iter()
            .map(|message| Line::raw(message.clone()))
            .collect()
    };
    f.render_widget(
        Paragraph::new(lines)
            .style(Style::default().fg(color))
            .block(Block::default().borders(Borders::ALL).title(title)),
        area,
    );
}
