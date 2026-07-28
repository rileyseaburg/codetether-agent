//! Preparation pipeline for user-like actions.

use crate::browser_agent::action_checks::ActionabilityReport;
use crate::browser_agent::hit_target::HitTarget;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::resolve::Resolved;
use crate::browser_agent::{actionability, hit, resolve, scroll};

pub(crate) fn click(
    page: &mut BrowserPage,
    locator: &Locator,
) -> Result<(Resolved, ActionabilityReport), String> {
    let resolved = resolve::resolve(&page.session, page.viewport_width, locator)?;
    let target = hit_target(page, locator, resolved.bounds)?;
    let checks = actionability::click(locator, &resolved, &target)?;
    Ok((resolved, checks))
}

pub(crate) fn fill(
    page: &mut BrowserPage,
    locator: &Locator,
) -> Result<(Resolved, ActionabilityReport), String> {
    let resolved = resolve::resolve(&page.session, page.viewport_width, locator)?;
    let target = hit_target(page, locator, resolved.bounds)?;
    let checks = actionability::fill(locator, &resolved, &target)?;
    Ok((resolved, checks))
}

fn hit_target(
    page: &mut BrowserPage,
    locator: &Locator,
    bounds: crate::browser_agent::action::BoundingBox,
) -> Result<HitTarget, String> {
    let point = scroll::center(bounds);
    scroll::into_view(
        &mut page.session.scroll,
        bounds,
        page.viewport_width,
        page.viewport_height,
    );
    hit::target_at(&page.session, page.viewport_width, point.0, point.1).ok_or_else(|| {
        format!(
            "locator {:?} failed actionability check receives_pointer: hit nothing",
            locator.kind
        )
    })
}
