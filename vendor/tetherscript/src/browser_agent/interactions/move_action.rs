//! Pointer movement routed through capture state.

use crate::browser_agent::action::ActionReport;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::{prepare, retry};
use crate::browser_session::TraceEvent;

impl BrowserPage {
    /// Move the pointer toward the locator, honoring active capture.
    pub fn pointer_move(&mut self, locator: &Locator) -> Result<ActionReport, String> {
        let (resolved, checks) = retry::run(self, "pointer_move", locator, |page| {
            prepare::click(page, locator)
        })?;
        let path = self
            .pointer_capture
            .as_ref()
            .unwrap_or(&resolved.dom.path)
            .clone();
        let script = format!(
            "{}{}",
            super::pointer_events::dispatch(&path, "pointermove", 0, resolved.bounds),
            super::pointer_events::mouse(&path, "mousemove", 0, resolved.bounds)
        );
        self.eval_js(&script)?;
        self.session.trace.push(TraceEvent::new(
            "pointer_move",
            format!("{:?}", locator.kind),
        ));
        Ok(ActionReport::new(
            "pointer_move",
            format!("{:?}", locator.kind),
            resolved.bounds,
            checks,
        ))
    }
}
