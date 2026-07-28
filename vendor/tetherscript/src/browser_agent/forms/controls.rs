//! DOM validation helpers for form-control actions.

use super::options;
use crate::browser::Element;
use crate::browser_agent::locator::Locator;

pub(crate) fn checkable(locator: &Locator, element: &Element) -> Result<String, String> {
    if !element.tag.eq_ignore_ascii_case("input") {
        return Err(fail(locator, "element is not an input"));
    }
    let kind = element
        .attrs
        .get("type")
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_else(|| "text".into());
    if matches!(kind.as_str(), "checkbox" | "radio") {
        Ok(kind)
    } else {
        Err(fail(locator, "input is not checkbox or radio"))
    }
}

pub(crate) fn selectable(locator: &Locator, element: &Element, value: &str) -> Result<(), String> {
    if !element.tag.eq_ignore_ascii_case("select") {
        return Err(fail(locator, "element is not a select"));
    }
    if options::has(element, value) {
        Ok(())
    } else {
        Err(fail(
            locator,
            &format!("select has no option value {value:?}"),
        ))
    }
}

fn fail(locator: &Locator, detail: &str) -> String {
    format!(
        "locator {:?} cannot perform form action: {detail}",
        locator.kind
    )
}
