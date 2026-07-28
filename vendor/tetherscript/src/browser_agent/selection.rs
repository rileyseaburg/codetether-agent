//! Agent-facing text selection helpers.

use crate::browser_agent::action::ActionReport;
use crate::browser_agent::action_checks::ActionabilityReport;
use crate::browser_agent::keyboard_escape::node;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::{resolve, retry};
use crate::browser_session::TraceEvent;

impl BrowserPage {
    /// Select all text contents inside the element matched by `locator`.
    pub fn select_contents(&mut self, locator: &Locator) -> Result<ActionReport, String> {
        let (resolved, checks) = retry::run(self, "select_contents", locator, |page| {
            let resolved = resolve::resolve(&page.session, page.viewport_width, locator)?;
            Ok((resolved, ActionabilityReport::new(false)))
        })?;
        let script = format!(
            "let n={}; let r=document.createRange(); r.selectNodeContents(n); \
             let s=document.getSelection(); s.removeAllRanges(); s.addRange(r); true;",
            node(&resolved.dom.path)
        );
        self.eval_js(&script)?;
        self.session.trace.push(TraceEvent::new(
            "select_contents",
            format!("{:?}", locator.kind),
        ));
        Ok(ActionReport::new(
            "select_contents",
            format!("{:?}", locator.kind),
            resolved.bounds,
            checks,
        ))
    }

    /// Return the current deterministic DOM selection text.
    pub fn selection_text(&mut self) -> Result<String, String> {
        Ok(self
            .eval_js("document.getSelection().toString();")?
            .display())
    }
}
