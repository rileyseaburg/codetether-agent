//! Select-specific pointer target checks.

use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::resolve::Resolved;
use crate::browser_agent::{hit, scroll};

pub(crate) fn receive_pointer(
    page: &mut BrowserPage,
    locator: &Locator,
    resolved: &Resolved,
) -> Result<(), String> {
    let point = scroll::center(resolved.bounds);
    scroll::into_view(
        &mut page.session.scroll,
        resolved.bounds,
        page.viewport_width,
        page.viewport_height,
    );
    let target = hit::target_at(&page.session, page.viewport_width, point.0, point.1)
        .ok_or_else(|| fail(locator, "hit nothing"))?;
    if target.path.starts_with(&resolved.dom.path) {
        Ok(())
    } else {
        Err(fail(locator, &format!("hit {}", target.label)))
    }
}

fn fail(locator: &Locator, detail: &str) -> String {
    format!(
        "locator {:?} failed actionability check receives_pointer: {detail}",
        locator.kind
    )
}
