//! Page-level pointer and mouse actions.

#[cfg(test)]
#[path = "pointer_event_tests.rs"]
mod pointer_event_tests;
#[path = "pointer_script.rs"]
mod pointer_script;

use crate::browser_agent::action::{ActionReport, BoundingBox};
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::{prepare, retry};
use crate::browser_session::TraceEvent;

impl BrowserPage {
    /// Move the pointer over the element matched by `locator`.
    pub fn hover(&mut self, locator: &Locator) -> Result<ActionReport, String> {
        self.pointer("hover", locator, pointer_script::hover)
    }

    /// Press the primary mouse button on the element matched by `locator`.
    pub fn mouse_down(&mut self, locator: &Locator) -> Result<ActionReport, String> {
        self.pointer("mouse_down", locator, pointer_script::mouse_down)
    }

    /// Release the primary mouse button on the element matched by `locator`.
    pub fn mouse_up(&mut self, locator: &Locator) -> Result<ActionReport, String> {
        self.pointer("mouse_up", locator, pointer_script::mouse_up)
    }

    /// Double-click the element matched by `locator`.
    pub fn double_click(&mut self, locator: &Locator) -> Result<ActionReport, String> {
        self.pointer("double_click", locator, pointer_script::double_click)
    }

    fn pointer(
        &mut self,
        action: &str,
        locator: &Locator,
        script: fn(&[usize], BoundingBox) -> String,
    ) -> Result<ActionReport, String> {
        let (resolved, checks) =
            retry::run(self, action, locator, |page| prepare::click(page, locator))?;
        self.eval_js(&script(&resolved.dom.path, resolved.bounds))?;
        self.session
            .trace
            .push(TraceEvent::new(action, format!("{:?}", locator.kind)));
        Ok(ActionReport::new(
            action,
            format!("{:?}", locator.kind),
            resolved.bounds,
            checks,
        ))
    }
}
