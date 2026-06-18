//! Rendering helpers: width computation, alignment, and table assembly.

#[path = "message_formatter_table_align.rs"]
pub(super) mod align;
#[path = "message_formatter_table_cellstyle.rs"]
pub(super) mod cellstyle;
#[path = "message_formatter_table_render_draw.rs"]
mod draw_mod;
#[path = "message_formatter_table_inline.rs"]
pub(super) mod inline;
#[path = "message_formatter_table_widths.rs"]
mod widths_mod;

use super::parse;
use super::util::w;
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

/// Compute display widths per column, capped so the table fits `max_width`.
fn column_widths(
    header: &[String],
    body: &[Vec<String>],
    cols: usize,
    max_width: usize,
) -> Vec<usize> {
    let mut widths = vec![0usize; cols];
    let mut scan = |row: &[String]| {
        for (i, cell) in row.iter().enumerate() {
            if i < cols {
                widths[i] = widths[i].max(w(cell)).max(3);
            }
        }
    };
    scan(header);
    for row in body {
        scan(row);
    }
    widths_mod::cap_widths(&mut widths, max_width);
    widths
}
