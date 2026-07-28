//! Scroll helpers for viewport-aware page actions.

use crate::browser_agent::action::BoundingBox;
use crate::browser_session::ScrollState;

pub(crate) fn center(bounds: BoundingBox) -> (i64, i64) {
    (bounds.x + bounds.width / 2, bounds.y + bounds.height / 2)
}

pub(crate) fn into_view(
    scroll: &mut ScrollState,
    bounds: BoundingBox,
    viewport_width: i64,
    viewport_height: i64,
) {
    let (center_x, center_y) = center(bounds);
    scroll.x = axis_scroll(scroll.x, center_x, viewport_width);
    scroll.y = axis_scroll(scroll.y, center_y, viewport_height);
}

fn axis_scroll(current: i64, center: i64, extent: i64) -> i64 {
    if extent <= 0 {
        return current.max(0);
    }
    if center < current || center >= current + extent {
        (center - extent / 2).max(0)
    } else {
        current.max(0)
    }
}
