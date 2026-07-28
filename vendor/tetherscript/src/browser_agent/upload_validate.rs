//! Validation helpers for file-input upload actions.

use crate::browser::Element;
use crate::browser_agent::locator::Locator;
use crate::browser_agent::FilePayload;

pub(crate) fn file_input(
    locator: &Locator,
    element: &Element,
    files: &[FilePayload],
) -> Result<(), String> {
    if !element.tag.eq_ignore_ascii_case("input") {
        return Err(fail(locator, "element is not input[type=file]"));
    }
    if input_type(element) != "file" {
        return Err(fail(locator, "input type is not file"));
    }
    if element.attrs.contains_key("disabled") {
        return Err(fail(locator, "input is disabled"));
    }
    if files.len() > 1 && !element.attrs.contains_key("multiple") {
        return Err(fail(
            locator,
            "multiple files require the multiple attribute",
        ));
    }
    Ok(())
}

fn input_type(element: &Element) -> String {
    element
        .attrs
        .get("type")
        .map(|value| value.to_ascii_lowercase())
        .unwrap_or_else(|| "text".into())
}

fn fail(locator: &Locator, detail: &str) -> String {
    format!(
        "locator {:?} cannot set input files: {detail}",
        locator.kind
    )
}
