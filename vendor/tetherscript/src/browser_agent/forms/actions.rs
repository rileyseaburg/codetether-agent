//! Public form-control action methods.

use super::{ready, report, scripts};
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::ActionReport;

impl BrowserPage {
    /// Ensure a checkbox or radio input is checked.
    pub fn check(&mut self, locator: &Locator) -> Result<ActionReport, String> {
        let (resolved, checks, _) = ready::check(self, "check", locator)?;
        if !resolved.dom.element.attrs.contains_key("checked") {
            self.eval_js(&scripts::check(&resolved.dom.path))?;
        }
        Ok(report::finish(
            self,
            "check",
            locator,
            resolved.bounds,
            checks,
        ))
    }

    /// Ensure a checkbox or radio input is unchecked.
    pub fn uncheck(&mut self, locator: &Locator) -> Result<ActionReport, String> {
        let (resolved, checks, kind) = ready::check(self, "uncheck", locator)?;
        if resolved.dom.element.attrs.contains_key("checked") {
            self.eval_js(&scripts::uncheck(&resolved.dom.path, &kind))?;
        }
        Ok(report::finish(
            self,
            "uncheck",
            locator,
            resolved.bounds,
            checks,
        ))
    }

    /// Select an option value on a `<select>` control.
    pub fn select_option(
        &mut self,
        locator: &Locator,
        value: &str,
    ) -> Result<ActionReport, String> {
        let (resolved, checks) = ready::select(self, locator, value)?;
        self.eval_js(&scripts::select_option(&resolved.dom.path, value))?;
        Ok(report::finish(
            self,
            "select_option",
            locator,
            resolved.bounds,
            checks,
        ))
    }
}
