//! Borderless table layout (modern TUI style: header + hairline rule).

use ratatui::text::Line;

use super::align::Align;

#[path = "clip.rs"]
mod clip_mod;
#[path = "data_row.rs"]
mod data;
#[path = "rowstyle.rs"]
mod rowstyle;
use data::data_row;
#[path = "rule.rs"]
mod rule;
use rule::rule_row;

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
