//! Draw the dim rule separating table headers and data.
use super::{GAP, INDENT, rowstyle::rule_style};
use crate::util::{line, span};
use ratatui::text::{Line, Span};

/// A dim hairline under the header, one segment per column.
pub(super) fn rule_row(widths: &[usize]) -> Line<'static> {
    let mut spans: Vec<Span<'static>> = vec![span(INDENT.to_string(), rule_style())];
    for (i, width) in widths.iter().enumerate() {
        spans.push(span("─".repeat(*width), rule_style()));
        if i + 1 < widths.len() {
            spans.push(span(GAP.to_string(), rule_style()));
        }
    }
    line(spans)
}
