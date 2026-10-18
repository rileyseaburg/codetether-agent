//! Per-cell styling helpers for table rows.

use ratatui::style::Style;

use super::super::super::util::header_style;
use super::super::cellstyle::cell_style;

/// Base style for a cell, applying diff coloring for non-header rows.
pub(super) fn cell_base(raw: &str, is_header: bool) -> Style {
    let base = header_or_default(is_header);
    if is_header {
        base
    } else {
        cell_style(raw, base)
    }
}

/// Header rows are bold; body rows use the default style.
pub(super) fn header_or_default(is_header: bool) -> Style {
    if is_header {
        header_style()
    } else {
        Style::default()
    }
}

/// Dim style for the hairline rule under the header.
pub(super) fn rule_style() -> Style {
    Style::default().fg(ratatui::style::Color::DarkGray)
}
