//! Select-specific actionability preparation.

use super::{controls, select_hit};
use crate::browser_agent::action_checks::ActionabilityReport;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::resolve::Resolved;
use crate::browser_agent::{resolve, retry};

pub(crate) fn run(
    page: &mut BrowserPage,
    locator: &Locator,
    value: &str,
) -> Result<(Resolved, ActionabilityReport), String> {
    retry::run(page, "select_option", locator, |page| {
        let resolved = resolve::resolve(&page.session, page.viewport_width, locator)?;
        controls::selectable(locator, &resolved.dom.element, value)?;
        if resolved.dom.element.attrs.contains_key("disabled") {
            return Err(format!(
                "locator {:?} failed actionability check enabled: element is disabled",
                locator.kind
            ));
        }
        select_hit::receive_pointer(page, locator, &resolved)?;
        Ok((resolved, ActionabilityReport::new(true)))
    })
}
