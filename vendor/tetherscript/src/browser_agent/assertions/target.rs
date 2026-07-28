//! Locator observation helpers for retryable assertions.

use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::query::{locate, DomMatch};

pub(crate) fn single(page: &BrowserPage, locator: &Locator) -> Result<DomMatch, String> {
    let matches = locate(&page.session.document, locator);
    if matches.is_empty() {
        return Err(format!("locator {:?} matched no elements", locator.kind));
    }
    if locator.strict && matches.len() != 1 {
        return Err(format!(
            "locator {:?} matched {} elements",
            locator.kind,
            matches.len()
        ));
    }
    Ok(matches[0].clone())
}

pub(crate) fn count(page: &BrowserPage, locator: &Locator) -> usize {
    locate(&page.session.document, locator).len()
}
