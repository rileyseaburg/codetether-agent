//! High-level flexbox layout algorithm.

use super::align::{align_item, line_cross_positions};
use super::container::FlexContainer;
use super::distribute::distribute_line;
use super::item::{FlexItem, PositionedFlexItem, Rect, Size};
use super::position::main_positions;
use super::wrap::create_lines;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlexConstraints {
    pub available: Size,
}

pub fn layout_flex(
    c: &FlexContainer,
    children: &[FlexItem],
    constraints: FlexConstraints,
) -> Vec<PositionedFlexItem> {
    let main_avail = if c.is_row() {
        constraints.available.width
    } else {
        constraints.available.height
    };
    let cross_avail = if c.is_row() {
        constraints.available.height
    } else {
        constraints.available.width
    };
    let main_gap = if c.is_row() {
        c.gap.column_gap
    } else {
        c.gap.row_gap
    };
    let cross_gap = if c.is_row() {
        c.gap.row_gap
    } else {
        c.gap.column_gap
    };

    let mut lines = create_lines(c, children, main_avail, main_gap);
    for line in &mut lines {
        distribute_line(c, line, children, main_avail, main_gap);
    }

    let cross_positions = line_cross_positions(&lines, cross_avail, cross_gap, c.align_content);
    let mut out = Vec::with_capacity(children.len());

    for (line, cross_pos) in lines.iter().zip(cross_positions) {
        let mains = main_positions(line, main_avail, main_gap, c.justify_content);

        for (li, main_pos) in line.items.iter().zip(mains) {
            let item = &children[li.index];
            let visual_main = if c.is_reverse() {
                main_avail - main_pos - li.target
            } else {
                main_pos
            };

            let (cross_offset, cross_size) = align_item(c, item, line.cross_size);
            let visual_cross = cross_pos + cross_offset;

            let rect = if c.is_row() {
                Rect {
                    x: visual_main,
                    y: visual_cross,
                    width: li.target,
                    height: cross_size,
                }
            } else {
                Rect {
                    x: visual_cross,
                    y: visual_main,
                    width: cross_size,
                    height: li.target,
                }
            };

            out.push(PositionedFlexItem {
                index: li.index,
                rect,
            });
        }
    }

    out.sort_by_key(|p| p.index);
    out
}
