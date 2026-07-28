//! Agent-visible pointer capture bookkeeping.

use crate::browser_agent::action::ActionReport;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::{prepare, retry};
use crate::browser_session::TraceEvent;

impl BrowserPage {
    /// Capture subsequent pointer dispatches for the matched element.
    pub fn set_pointer_capture(&mut self, locator: &Locator) -> Result<ActionReport, String> {
        let (resolved, checks) = retry::run(self, "set_pointer_capture", locator, |page| {
            prepare::click(page, locator)
        })?;
        self.pointer_capture = Some(resolved.dom.path.clone());
        self.eval_js(&super::pointer_events::dispatch(
            &resolved.dom.path,
            "gotpointercapture",
            0,
            resolved.bounds,
        ))?;
        self.session.trace.push(TraceEvent::new(
            "set_pointer_capture",
            format!("{:?}", locator.kind),
        ));
        Ok(ActionReport::new(
            "set_pointer_capture",
            format!("{:?}", locator.kind),
            resolved.bounds,
            checks,
        ))
    }

    /// Release pointer capture when the locator owns it.
    pub fn release_pointer_capture(&mut self, locator: &Locator) -> Result<bool, String> {
        let (resolved, _) = retry::run(self, "release_pointer_capture", locator, |page| {
            prepare::click(page, locator)
        })?;
        if self.pointer_capture.as_ref() != Some(&resolved.dom.path) {
            return Ok(false);
        }
        self.pointer_capture = None;
        self.eval_js(&super::pointer_events::dispatch(
            &resolved.dom.path,
            "lostpointercapture",
            0,
            resolved.bounds,
        ))?;
        Ok(true)
    }
}
