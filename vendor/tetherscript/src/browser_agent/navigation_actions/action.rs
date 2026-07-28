//! Click action integration with navigation defaults.

use crate::browser_agent::action::ActionReport;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::{downloads, prepare, retry};
use crate::browser_session::TraceEvent;

impl BrowserPage {
    /// Click an actionable element and apply anchor/form navigation defaults.
    pub fn click(&mut self, locator: &Locator) -> Result<ActionReport, String> {
        let (resolved, checks) =
            retry::run(self, "click", locator, |page| prepare::click(page, locator))?;
        let is_download = downloads::is_anchor_download(&resolved);
        let script = if is_download {
            downloads::click_script(&resolved.dom.path)
        } else {
            super::click_user::script(&resolved.dom.path, resolved.bounds)
        };
        let result = self.eval_js(&script)?;
        if is_download {
            downloads::record_anchor_download(self, &resolved, &result);
        } else {
            super::click::after_click(self, &resolved, &result)?;
        }
        self.session
            .trace
            .push(TraceEvent::new("click", format!("{:?}", locator.kind)));
        Ok(ActionReport::new(
            "click",
            format!("{:?}", locator.kind),
            resolved.bounds,
            checks,
        ))
    }
}
