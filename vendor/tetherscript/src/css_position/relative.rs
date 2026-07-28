//! Relative positioning.
use super::types::PositionedElement;

pub struct RelativePositioner;

impl RelativePositioner {
    pub fn compute<E>(el: &mut PositionedElement<E>) {
        let dx = el.offsets.left.unwrap_or(0.0) - el.offsets.right.unwrap_or(0.0);
        let dy = el.offsets.top.unwrap_or(0.0) - el.offsets.bottom.unwrap_or(0.0);
        el.computed_x = el.normal_x + dx;
        el.computed_y = el.normal_y + dy;
    }
}
