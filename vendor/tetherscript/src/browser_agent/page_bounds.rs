//! Locator convenience helpers for browser pages.

use crate::browser_agent::action::BoundingBox;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Return the locator unchanged for fluent call sites.
    pub fn locator(&self, locator: Locator) -> Locator {
        locator
    }

    /// Resolve a locator and return its current layout bounds.
    pub fn bounding_box(&self, locator: &Locator) -> Result<BoundingBox, String> {
        crate::browser_agent::resolve::resolve(&self.session, self.viewport_width, locator)
            .map(|r| r.bounds)
    }
}
