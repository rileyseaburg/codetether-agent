//! Absolute positioning.
use super::containing::ContainingBlock;
use super::types::PositionedElement;

pub struct AbsolutePositioner;

impl AbsolutePositioner {
    pub fn compute<E>(el: &mut PositionedElement<E>, cb: ContainingBlock) {
        let r = cb.rect;
        el.computed_x = if let Some(left) = el.offsets.left {
            r.x + left + el.margin.left
        } else if let Some(right) = el.offsets.right {
            r.x + r.width - right - el.width - el.margin.right
        } else {
            el.normal_x
        };
        el.computed_y = if let Some(top) = el.offsets.top {
            r.y + top + el.margin.top
        } else if let Some(bottom) = el.offsets.bottom {
            r.y + r.height - bottom - el.height - el.margin.bottom
        } else {
            el.normal_y
        };
    }
}
