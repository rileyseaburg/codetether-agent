//! Complete language-server diagnostics for scrollable approval details.

use crate::tui::app::state::approval_queue::{ApprovalReport, ApprovalReportState};
use ratatui::{
    style::Stylize,
    text::{Line, Text},
};

/// Keep all diagnostics rather than reserving a clipped, fixed-height panel.
pub(super) fn lines(report: &ApprovalReport) -> Vec<Line<'static>> {
    let title = match report.state {
        ApprovalReportState::Checking => "LSP · checking proposed content…".yellow(),
        ApprovalReportState::Clean => "LSP · no diagnostics".green(),
        ApprovalReportState::Issues => "LSP · proposed-content diagnostics".red(),
        ApprovalReportState::Unavailable => "LSP · unavailable".dim(),
        ApprovalReportState::NotRequested => return Vec::new(),
    };
    let mut lines = vec![Line::default(), Line::from(title.bold())];
    lines.extend(
        report
            .messages
            .iter()
            .flat_map(|message| Text::raw(message.clone()).lines),
    );
    lines
}
