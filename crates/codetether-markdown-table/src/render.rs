//! Rendering helpers: width computation, alignment, and table assembly.

#[path = "align.rs"]
pub(super) mod align;
#[path = "cellstyle.rs"]
pub(super) mod cellstyle;
#[path = "render_draw.rs"]
mod draw_mod;
#[path = "inline.rs"]
pub(super) mod inline;
#[path = "widths.rs"]
mod widths_mod;

use super::parse;
#[path = "columns.rs"]
mod columns;
use columns::column_widths;
use ratatui::text::Line;

/// Assemble a table from raw rows: split cells, size, align, and draw.
pub(super) fn table(buf: &[String], max_width: usize) -> Option<Vec<Line<'static>>> {
    let header = parse::split_cells(&buf[0]);
    let body: Vec<Vec<String>> = buf[2..].iter().map(|r| parse::split_cells(r)).collect();
    let cols = header
        .len()
        .max(body.iter().map(Vec::len).max().unwrap_or(0));
    if cols == 0 {
        return None;
    }
    let widths = column_widths(&header, &body, cols, max_width);
    let aligns = align::parse_aligns(&buf[1]);
    Some(draw_mod::draw(&header, &body, &widths, &aligns))
}
