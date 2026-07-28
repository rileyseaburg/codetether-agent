//! Public page drag-and-drop action.

use crate::browser_agent::action::ActionReport;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::{prepare, retry};
use crate::browser_session::TraceEvent;

use super::drag_script;

impl BrowserPage {
    /// Drag the source element to the target element.
    pub fn drag_to(&mut self, source: &Locator, target: &Locator) -> Result<ActionReport, String> {
        let (source_resolved, checks) =
            retry::run(self, "drag_to", source, |page| prepare::click(page, source))?;
        let (target_resolved, _) = retry::run(self, "drop_target", target, |page| {
            prepare::click(page, target)
        })?;
        self.eval_js(&drag_script::drag(
            &source_resolved.dom.path,
            &target_resolved.dom.path,
            source_resolved.bounds,
            target_resolved.bounds,
        ))?;
        let label = format!("{:?} -> {:?}", source.kind, target.kind);
        self.session.trace.push(TraceEvent::new("drag_to", &label));
        Ok(ActionReport::new(
            "drag_to",
            label,
            source_resolved.bounds,
            checks,
        ))
    }
}
