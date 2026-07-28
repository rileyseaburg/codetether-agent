//! Action reports and trace entries for form-control actions.

use crate::browser_agent::action::{ActionReport, BoundingBox};
use crate::browser_agent::action_checks::ActionabilityReport;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_session::TraceEvent;

pub(crate) fn finish(
    page: &mut BrowserPage,
    action: &str,
    locator: &Locator,
    bounds: BoundingBox,
    checks: ActionabilityReport,
) -> ActionReport {
    page.session
        .trace
        .push(TraceEvent::new(action, format!("{:?}", locator.kind)));
    ActionReport::new(action, format!("{:?}", locator.kind), bounds, checks)
}
