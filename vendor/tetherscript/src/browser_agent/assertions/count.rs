//! Count assertion observations.

use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;

use super::{retry, target};

pub(crate) fn exact(
    page: &mut BrowserPage,
    locator: &Locator,
    expected: usize,
) -> Result<(), String> {
    retry::run(page, "expect_count", |page| {
        let actual = target::count(page, locator);
        if actual == expected {
            Ok(())
        } else {
            Err(format!("expected count {expected}, last count {actual}"))
        }
    })
}
