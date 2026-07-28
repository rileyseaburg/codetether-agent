//! Sticky positioning.
use super::containing::ContainingBlock;
use super::types::PositionedElement;

pub struct StickyOffsetResolver;

impl StickyOffsetResolver {
    pub fn compute<E>(
        el: &mut PositionedElement<E>,
        cb: ContainingBlock,
        viewport: ContainingBlock,
        scroll_x: f32,
        scroll_y: f32,
    ) {
        let mut x = el.normal_x - scroll_x;
        let mut y = el.normal_y - scroll_y;
        let c = cb.rect;
        let v = viewport.rect;

        if let Some(top) = el.offsets.top {
            y = y.max(v.y + top);
        }
        if let Some(bottom) = el.offsets.bottom {
            y = y.min(v.y + v.height - bottom - el.height);
        }
        if let Some(left) = el.offsets.left {
            x = x.max(v.x + left);
        }
        if let Some(right) = el.offsets.right {
            x = x.min(v.x + v.width - right - el.width);
        }
        x = x.clamp(c.x, c.x + c.width - el.width);
        y = y.clamp(c.y, c.y + c.height - el.height);
        el.computed_x = x;
        el.computed_y = y;
    }
}
