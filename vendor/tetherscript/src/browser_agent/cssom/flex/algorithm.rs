//! Top-level flex layout algorithm.

use super::{align, parse, position, resolve, types::*, wrap_lines};
use std::collections::HashMap;

/// Perform flex layout on a container's children.
///
/// Returns positioned `FlexItem`s with x, y, width, height set.
pub fn perform_flex_layout(
    container_styles: &HashMap<String, String>,
    container_width: i64,
    children_styles: &[HashMap<String, String>],
    child_sizes: &[(i64, i64)],
) -> Vec<FlexItem> {
    let dir = parse::direction(container_styles);
    let row = resolve::is_row(dir);
    let main = if row {
        container_width
    } else {
        parse::len(container_styles, "height").unwrap_or(container_width)
    };
    let cross = if row {
        align::container_cross(container_styles, row, 0)
    } else {
        parse::len(container_styles, "width").unwrap_or(container_width)
    };
    let mut items = resolve::collect(children_styles, child_sizes, dir);
    let flex_lines = wrap_lines::lines(&mut items, row, main, parse::wrap(container_styles));
    let mut off = 0;
    for (s, e, line_cross) in flex_lines {
        position::position_line(
            &mut items[s..e],
            row,
            main,
            parse::justify(container_styles),
            resolve::is_reverse(dir),
        );
        let line_styles = &children_styles[s..e.min(children_styles.len())];
        let c = if cross > 0 { cross } else { line_cross };
        align::align_line(
            &mut items[s..e],
            line_styles,
            row,
            c,
            parse::align_items(container_styles),
            off,
        );
        off += line_cross;
    }
    items
}
