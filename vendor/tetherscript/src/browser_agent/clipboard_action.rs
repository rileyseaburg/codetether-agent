//! Clipboard-backed page actions.

#[path = "clipboard.rs"]
mod clipboard;
#[path = "clipboard_script.rs"]
mod clipboard_script;
#[path = "clipboard_source.rs"]
mod clipboard_source;

use crate::browser_agent::action::ActionReport;
use crate::browser_agent::action_checks::ActionabilityReport;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::{prepare, resolve, retry};
use crate::browser_session::TraceEvent;

impl BrowserPage {
    /// Copy the resolved element's text into the page clipboard.
    pub fn copy_text(&mut self, locator: &Locator) -> Result<ActionReport, String> {
        let resolved = retry::run(self, "copy_text", locator, |page| {
            resolve::resolve(&page.session, page.viewport_width, locator)
        })?;
        let text = clipboard_source::text(&resolved.dom.element);
        self.eval_js(&clipboard_script::copy(&resolved.dom.path))?;
        self.write_clipboard(text);
        self.session
            .trace
            .push(TraceEvent::new("copy_text", format!("{:?}", locator.kind)));
        Ok(ActionReport::new(
            "copy_text",
            format!("{:?}", locator.kind),
            resolved.bounds,
            ActionabilityReport::new(false),
        ))
    }

    /// Paste the page clipboard into an editable element.
    pub fn paste(&mut self, locator: &Locator) -> Result<ActionReport, String> {
        let (resolved, checks) =
            retry::run(self, "paste", locator, |page| prepare::fill(page, locator))?;
        self.eval_js(&clipboard_script::paste(
            &resolved.dom.path,
            &self.read_clipboard(),
        ))?;
        self.session
            .trace
            .push(TraceEvent::new("paste", format!("{:?}", locator.kind)));
        Ok(ActionReport::new(
            "paste",
            format!("{:?}", locator.kind),
            resolved.bounds,
            checks,
        ))
    }
}
