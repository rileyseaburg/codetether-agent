//! Complete request details sharing one scrollable viewport.

use crate::tui::app::state::approval_queue::ApprovalSnapshot;
use ratatui::{style::Stylize, text::Line};

/// Include every command, explanation, diagnostic and reviewer finding.
pub(super) fn lines(item: &ApprovalSnapshot) -> Vec<Line<'static>> {
    let mut lines = super::approval_overlay_text::lines(item);
    lines.push(Line::default());
    lines.push(Line::from("Invocation".cyan().bold()));
    lines.extend(super::approval_diff::lines(
        item.preview.as_deref().unwrap_or(&item.resource),
    ));
    lines.extend(super::approval_overlay_lsp::lines(&item.report));
    lines.extend(super::approval_overlay_review::lines(item.review.as_ref()));
    lines
}
