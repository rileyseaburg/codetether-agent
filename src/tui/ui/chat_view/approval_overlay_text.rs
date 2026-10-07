//! Scrollable request metadata for the approval popup.

use crate::tui::app::state::approval_queue::ApprovalSnapshot;
use ratatui::{
    style::Stylize,
    text::{Line, Text},
};

/// Compose metadata without clipping long reasons or justifications.
pub(super) fn lines(item: &ApprovalSnapshot) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from(
            format!("{} wants to {}", item.tool, item.action)
                .yellow()
                .bold(),
        ),
        Line::from(format!("id: {}", item.id).dim()),
    ];
    lines.extend(Text::raw(format!("→ {}", item.reason)).lines);
    match &item.justification {
        Some(justification) => {
            lines.extend(Text::raw(format!("because: {justification}")).lines);
        }
        None => lines.push(Line::from("because: (no justification given)".red().dim())),
    }
    lines
}
