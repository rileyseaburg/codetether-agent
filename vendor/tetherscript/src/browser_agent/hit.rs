//! Hit testing for agent pointer actions.

use crate::browser::{find_layout_box_at_path, layout_document};
use crate::browser_agent::hit_style::{bounds_for, pointer_enabled, z_index};
use crate::browser_agent::hit_target::HitTarget;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::query::locate;
use crate::browser_session::BrowserSession;

pub(crate) fn target_at(
    session: &BrowserSession,
    viewport_width: i64,
    x: i64,
    y: i64,
) -> Option<HitTarget> {
    let layout = layout_document(&session.document, &session.css, viewport_width);
    let locator = Locator::css("*").relaxed();
    let mut best: Option<(i64, usize, HitTarget)> = None;
    for (order, dom) in locate(&session.document, &locator).into_iter().enumerate() {
        let Some(layout_box) = find_layout_box_at_path(&layout, &dom.path) else {
            continue;
        };
        let bounds = bounds_for(layout_box);
        if !bounds.visible() || !pointer_enabled(layout_box) || !contains(bounds, x, y) {
            continue;
        }
        let item = (
            z_index(layout_box),
            order,
            HitTarget::new(dom.path, &dom.element),
        );
        if best
            .as_ref()
            .is_none_or(|old| item.0 > old.0 || item.0 == old.0 && item.1 > old.1)
        {
            best = Some(item);
        }
    }
    best.map(|(_, _, target)| target)
}

fn contains(bounds: crate::browser_agent::action::BoundingBox, x: i64, y: i64) -> bool {
    x >= bounds.x && y >= bounds.y && x < bounds.x + bounds.width && y < bounds.y + bounds.height
}
