//! Pointer capture read APIs.

use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::{prepare, retry};

impl BrowserPage {
    /// Return whether the matched element currently owns pointer capture.
    pub fn has_pointer_capture(&mut self, locator: &Locator) -> Result<bool, String> {
        let (resolved, _) = retry::run(self, "has_pointer_capture", locator, |page| {
            prepare::click(page, locator)
        })?;
        Ok(self.pointer_capture.as_ref() == Some(&resolved.dom.path))
    }
}
