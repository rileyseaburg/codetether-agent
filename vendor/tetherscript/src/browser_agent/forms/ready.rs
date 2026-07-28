//! Retry-backed preparation for form-control actions.

use super::{controls, select_ready};
use crate::browser_agent::action_checks::ActionabilityReport;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::resolve::Resolved;
use crate::browser_agent::{prepare, retry};

pub(crate) fn check(
    page: &mut BrowserPage,
    action: &str,
    locator: &Locator,
) -> Result<(Resolved, ActionabilityReport, String), String> {
    retry::run(page, action, locator, |page| {
        let (resolved, checks) = prepare::click(page, locator)?;
        let kind = controls::checkable(locator, &resolved.dom.element)?;
        Ok((resolved, checks, kind))
    })
}

pub(crate) fn select(
    page: &mut BrowserPage,
    locator: &Locator,
    value: &str,
) -> Result<(Resolved, ActionabilityReport), String> {
    select_ready::run(page, locator, value)
}
