//! Actionability checks for agent page actions.

use crate::browser_agent::action_checks::ActionabilityReport;
use crate::browser_agent::editable::editable;
use crate::browser_agent::hit_target::HitTarget;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::resolve::Resolved;

pub(crate) fn click(
    locator: &Locator,
    resolved: &Resolved,
    hit: &HitTarget,
) -> Result<ActionabilityReport, String> {
    common(locator, resolved, hit)?;
    Ok(ActionabilityReport::new(false))
}

pub(crate) fn fill(
    locator: &Locator,
    resolved: &Resolved,
    hit: &HitTarget,
) -> Result<ActionabilityReport, String> {
    common(locator, resolved, hit)?;
    if !editable(&resolved.dom.element) {
        return Err(fail(locator, "editable", "element is not fillable"));
    }
    Ok(ActionabilityReport::new(true))
}

fn common(locator: &Locator, resolved: &Resolved, hit: &HitTarget) -> Result<(), String> {
    if !resolved.bounds.visible() {
        return Err(fail(locator, "visible", "element has an empty layout box"));
    }
    if resolved.dom.element.attrs.contains_key("disabled") {
        return Err(fail(locator, "enabled", "element is disabled"));
    }
    if hit.path != resolved.dom.path {
        return Err(fail(
            locator,
            "receives_pointer",
            &format!("hit {}", hit.label),
        ));
    }
    Ok(())
}

fn fail(locator: &Locator, check: &str, detail: &str) -> String {
    format!(
        "locator {:?} failed actionability check {check}: {detail}",
        locator.kind
    )
}
