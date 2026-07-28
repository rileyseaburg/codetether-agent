//! Containing block content-box math.

use super::super::types::{PositionedElement, Rect};
use super::ContainingBlock;

pub fn resolve<E>(el: &PositionedElement<E>) -> ContainingBlock {
    let x = el.computed_x + el.border.left + el.padding.left;
    let y = el.computed_y + el.border.top + el.padding.top;
    let w = el.width - el.border.left - el.border.right - el.padding.left - el.padding.right;
    let h = el.height - el.border.top - el.border.bottom - el.padding.top - el.padding.bottom;
    ContainingBlock {
        rect: Rect {
            x,
            y,
            width: w.max(0.0),
            height: h.max(0.0),
        },
    }
}
