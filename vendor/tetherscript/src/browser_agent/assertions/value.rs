//! Form-value assertion observations.

use crate::browser::{self, Element, Node};
use crate::browser_agent::locator::Locator;
use crate::browser_agent::page::BrowserPage;

use super::{retry, target};

pub(crate) fn exact(
    page: &mut BrowserPage,
    locator: &Locator,
    expected: &str,
) -> Result<(), String> {
    retry::run(page, "expect_value", |page| {
        let actual = observed(page, locator)?;
        if actual == expected {
            Ok(())
        } else {
            Err(format!(
                "expected value {expected:?}, last value {actual:?}"
            ))
        }
    })
}

fn observed(page: &BrowserPage, locator: &Locator) -> Result<String, String> {
    let matched = target::single(page, locator)?;
    Ok(value_of(&matched.element))
}

fn value_of(element: &Element) -> String {
    if element.tag.eq_ignore_ascii_case("textarea") {
        return browser::text_content(&Node::Element(element.clone()));
    }
    element.attrs.get("value").cloned().unwrap_or_default()
}
