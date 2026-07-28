//! Text assertion observations.

use crate::browser::{self, Node};
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::text_match;

use super::{retry, target};

pub(crate) fn exact(
    page: &mut BrowserPage,
    locator: &Locator,
    expected: &str,
) -> Result<(), String> {
    retry::run(page, "expect_text", |page| {
        let actual = observed(page, locator)?;
        if same_text(&actual, expected) {
            Ok(())
        } else {
            Err(format!("expected text {expected:?}, last text {actual:?}"))
        }
    })
}

pub(crate) fn contains(
    page: &mut BrowserPage,
    locator: &Locator,
    expected: &str,
) -> Result<(), String> {
    retry::run(page, "expect_text_contains", |page| {
        let actual = observed(page, locator)?;
        if contains_text(&actual, expected) {
            Ok(())
        } else {
            Err(format!(
                "expected text containing {expected:?}, last text {actual:?}"
            ))
        }
    })
}

fn observed(page: &BrowserPage, locator: &Locator) -> Result<String, String> {
    let matched = target::single(page, locator)?;
    Ok(browser::text_content(&Node::Element(matched.element)))
}

fn same_text(actual: &str, expected: &str) -> bool {
    text_match::clean(actual) == text_match::clean(expected)
}

fn contains_text(actual: &str, expected: &str) -> bool {
    text_match::clean(actual).contains(&text_match::clean(expected))
}
