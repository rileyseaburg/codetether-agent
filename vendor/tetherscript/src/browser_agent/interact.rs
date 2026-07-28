//! User-like page interactions.

#[path = "focus.rs"]
pub mod focus;
#[path = "forms/mod.rs"]
mod forms;
#[path = "interactions/mod.rs"]
pub(crate) mod interactions;
#[path = "navigation_actions/mod.rs"]
mod navigation_actions;
#[path = "pointer.rs"]
mod pointer;
#[path = "pointer_event_fields.rs"]
pub(crate) mod pointer_event_fields;
#[path = "pointer_touch.rs"]
mod pointer_touch;
#[path = "pointer_wheel.rs"]
mod pointer_wheel;

use crate::browser_agent::action::ActionReport;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::{prepare, retry, script};
use crate::browser_session::TraceEvent;

impl BrowserPage {
    /// Fill an input-like element resolved by `locator`.
    pub fn fill(&mut self, locator: &Locator, text: &str) -> Result<ActionReport, String> {
        let (resolved, checks) =
            retry::run(self, "fill", locator, |page| prepare::fill(page, locator))?;
        self.eval_js(&script::fill(&resolved.dom.path, text))?;
        self.trace("fill", locator);
        Ok(ActionReport::new(
            "fill",
            format!("{:?}", locator.kind),
            resolved.bounds,
            checks,
        ))
    }

    fn trace(&mut self, action: &str, locator: &Locator) {
        self.session
            .trace
            .push(TraceEvent::new(action, format!("{:?}", locator.kind)));
    }
}
