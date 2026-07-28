//! Fixed positioning.
use super::absolute::AbsolutePositioner;
use super::containing::ContainingBlock;
use super::types::PositionedElement;

pub struct FixedPositioner;

impl FixedPositioner {
    pub fn compute<E>(el: &mut PositionedElement<E>, viewport: ContainingBlock) {
        AbsolutePositioner::compute(el, viewport);
    }
}
