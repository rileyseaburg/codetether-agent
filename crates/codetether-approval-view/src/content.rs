//! Complete request details sharing one scrollable viewport.

use codetether_approval_queue::ApprovalSnapshot;
use ratatui::{style::Stylize, text::Line};

/// Include every command, explanation, diagnostic and reviewer finding.
/// # Arguments
/// * `item` - Pending request snapshot with its analysis and review state.
/// # Returns
/// Owned styled lines containing all request details.
/// # Examples
/// ```
/// fn details(item: &codetether_approval_queue::ApprovalSnapshot) {
///     assert!(!codetether_approval_view::content_lines(item).is_empty());
/// }
/// ```
pub fn lines(item: &ApprovalSnapshot) -> Vec<Line<'static>> {
    let mut lines = crate::text::lines(item);
    lines.push(Line::default());
    lines.push(Line::from("Invocation".cyan().bold()));
    lines.extend(codetether_approval_diff::lines(
        item.preview.as_deref().unwrap_or(&item.resource),
    ));
    lines.extend(crate::lsp::lines(&item.report));
    lines.extend(crate::review::lines(item.review.as_ref()));
    lines
}
