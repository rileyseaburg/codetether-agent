//! Retryable page assertions.

#[path = "assertions/count.rs"]
mod count;
#[path = "assertions/retry.rs"]
mod retry;
#[path = "assertions/target.rs"]
mod target;
#[path = "assertions/text.rs"]
mod text;
#[path = "assertions/url.rs"]
mod url;
#[path = "assertions/value.rs"]
mod value;

use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Assert that `locator` becomes visible within the page wait budget.
    pub fn expect_visible(&mut self, locator: &Locator) -> Result<(), String> {
        self.wait_for_visible(locator).map(|_| ())
    }

    /// Assert that `locator` has exactly `expected` normalized text.
    pub fn expect_text(&mut self, locator: &Locator, expected: &str) -> Result<(), String> {
        text::exact(self, locator, expected)
    }

    /// Assert that `locator` has normalized text containing `expected`.
    pub fn expect_text_contains(
        &mut self,
        locator: &Locator,
        expected: &str,
    ) -> Result<(), String> {
        text::contains(self, locator, expected)
    }

    /// Assert that `locator` has exactly `expected` form value.
    pub fn expect_value(&mut self, locator: &Locator, expected: &str) -> Result<(), String> {
        value::exact(self, locator, expected)
    }

    /// Assert that `locator` resolves to `expected` elements.
    pub fn expect_count(&mut self, locator: &Locator, expected: usize) -> Result<(), String> {
        count::exact(self, locator, expected)
    }

    /// Assert that the current page URL contains `substring`.
    pub fn expect_url_contains(&mut self, substring: &str) -> Result<(), String> {
        url::contains(self, substring)
    }
}

#[cfg(test)]
#[path = "assertions/state_tests.rs"]
mod state_tests;
#[cfg(test)]
#[path = "assertions/text_tests.rs"]
mod text_tests;
