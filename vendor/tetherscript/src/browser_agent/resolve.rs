//! Resolve locators into actionable elements.

use crate::browser::{find_layout_box_at_path, layout_document};
use crate::browser_agent::action::BoundingBox;
use crate::browser_agent::hit_style::bounds_for;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::query::{locate, DomMatch};
use crate::browser_session::BrowserSession;

#[derive(Debug, Clone)]
pub(crate) struct Resolved {
    pub dom: DomMatch,
    pub bounds: BoundingBox,
}

pub(crate) fn resolve(
    session: &BrowserSession,
    viewport_width: i64,
    locator: &Locator,
) -> Result<Resolved, String> {
    let matches = locate(&session.document, locator);
    if matches.is_empty() {
        return Err(format!("locator {:?} matched no elements", locator.kind));
    }
    if locator.strict && matches.len() != 1 {
        return Err(format!(
            "locator {:?} matched {} elements",
            locator.kind,
            matches.len()
        ));
    }
    let dom = matches[0].clone();
    let layout = layout_document(&session.document, &session.css, viewport_width);
    let Some(layout_box) = find_layout_box_at_path(&layout, &dom.path) else {
        return Err(format!("locator {:?} is not visible", locator.kind));
    };
    let bounds = bounds_for(layout_box);
    if !bounds.visible() {
        return Err(format!("locator {:?} is not actionable", locator.kind));
    }
    Ok(Resolved { dom, bounds })
}
