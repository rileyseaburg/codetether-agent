//! Locator wait helpers for deterministic agent pages.

use crate::browser_agent::action::BoundingBox;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::{resolve, retry};

impl BrowserPage {
    /// Wait until `locator` resolves to a visible element.
    pub fn wait_for(&mut self, locator: &Locator) -> Result<BoundingBox, String> {
        self.wait_for_visible(locator)
    }

    /// Wait until `locator` resolves to a visible element.
    pub fn wait_for_visible(&mut self, locator: &Locator) -> Result<BoundingBox, String> {
        retry::run(self, "wait_for_visible", locator, |page| {
            resolve::resolve(&page.session, page.viewport_width, locator).map(|r| r.bounds)
        })
    }
}
