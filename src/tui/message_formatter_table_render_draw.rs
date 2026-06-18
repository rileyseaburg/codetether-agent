//! Borderless table layout (modern TUI style: header + hairline rule).

use ratatui::text::{Line, Span};

use super::super::util::{line, span};
use super::align::{Align, align};
use super::inline::cell_spans;

#[path = "message_formatter_table_clip.rs"]
mod clip_mod;
#[path = "message_formatter_table_rowstyle.rs"]
mod rowstyle;
use clip_mod::clip;
use rowstyle::{cell_base, header_or_default, rule_style};

/// Left indent and inter-column gap (both two spaces).
const GAP: &str = "  ";
const INDENT: &str = GAP;

/// Draw a borderless table: header row, dim rule, then data rows.
pub(crate) fn draw(
    header: &[String],
    body: &[Vec<String>],
    widths: &[usize],
    aligns: &[Align],
) -> Vec<Line<'static>> {
    let mut out = vec![data_row(header, widths, aligns, true), rule_row(widths)];
    for row in body {
        out.push(data_row(row, widths, aligns, false));
    }
    out
}

/// A dim hairline under the header, one segment per column.
fn rule_row(widths: &[usize]) -> Line<'static> {
    let mut spans: Vec<Span<'static>> = vec![span(INDENT.to_string(), rule_style())];
    for (i, width) in widths.iter().enumerate() {
        spans.push(span("─".repeat(*width), rule_style()));
        if i + 1 < widths.len() {
            spans.push(span(GAP.to_string(), rule_style()));
        }
    }
    line(spans)
}

/// A data row: indented, gap-separated, aligned, styled cells.
fn data_row(
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
