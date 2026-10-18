//! Align and style a header or body row.
use super::{
    GAP, INDENT,
    clip_mod::clip,
    rowstyle::{cell_base, header_or_default},
};
use crate::render::{
    align::{Align, align},
    inline::cell_spans,
};
use crate::util::{line, span};
use ratatui::text::{Line, Span};

/// A data row: indented, gap-separated, aligned, styled cells.
pub(super) fn data_row(
    cells: &[String],
    widths: &[usize],
    aligns: &[Align],
    is_header: bool,
) -> Line<'static> {
    let pad_style = header_or_default(is_header);
    let mut spans: Vec<Span<'static>> = vec![span(INDENT.to_string(), pad_style)];
    for (i, width) in widths.iter().enumerate() {
        let raw = cells.get(i).map(String::as_str).unwrap_or("");
        let a = aligns.get(i).copied().unwrap_or(Align::Left);
        let text = align(&clip(raw, *width), *width, a);
        spans.extend(cell_spans(&text, cell_base(raw, is_header)));
        if i + 1 < widths.len() {
            spans.push(span(GAP.to_string(), pad_style));
        }
    }
    line(spans)
}
